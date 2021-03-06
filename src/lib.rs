mod utils;

use js_sys::{Array, JsString};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use bstr::BString;

use gfa::{
    gfa::{Line, Orientation, GFA},
    parser::GFAParser,
};

macro_rules! log {
    ( $( $t:tt )*) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(module = "/util.js")]
extern "C" {
    fn immutable_closure(f: &dyn Fn(u32) -> String);
}

#[wasm_bindgen]
pub fn some_fun() {
    immutable_closure(&|x| format!("{}", x * 3));
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Segment = gfa::gfa::Segment<BString, ()>;
type Link = gfa::gfa::Link<BString, ()>;
type Path = gfa::gfa::Path<()>;

#[wasm_bindgen]
pub struct WrappedGFA {
    graph: GFA<BString, ()>,
}

impl WrappedGFA {
    fn parse(gfa_string: JsString) -> WrappedGFA {
        let parser: GFAParser<()> = GFAParser::new();
        let gfa_string: String = gfa_string.as_string().unwrap();
        let lines = gfa_string.lines().map(|l| l.as_bytes());
        let graph = parser.parse_all(lines);
        WrappedGFA { graph }
    }
}

#[wasm_bindgen]
pub struct JSlice {
    pub start: usize,
    pub len: usize,
    pub stride: usize,
}

impl JSlice {
    pub fn new(start: usize, len: usize, stride: usize) -> Self {
        JSlice { start, len, stride }
    }

    pub fn from_slice<T>(slice: &[T]) -> Self {
        let start = slice.as_ptr() as usize;
        let len = slice.len();
        let stride = std::mem::size_of::<T>();

        JSlice { start, len, stride }
    }
}

#[wasm_bindgen]
impl WrappedGFA {
    pub fn new_gfa() -> WrappedGFA {
        WrappedGFA {
            graph: Default::default(),
        }
    }

    pub fn each_segment(&self, f: &js_sys::Function) {
        let this = JsValue::null();
        for x in &self.graph.segments {
            let name = std::str::from_utf8(&x.name).unwrap();
            let name = JsValue::from(name);
            let seq = std::str::from_utf8(&x.sequence).unwrap();
            let seq = JsValue::from(seq);
            let _ = f.call2(&this, &name, &seq);
        }
    }

    pub fn paths(&self) -> JSlice {
        JSlice::from_slice(self.graph.paths.as_slice())
    }

    pub fn segments(&self) -> JSlice {
        JSlice::from_slice(self.graph.segments.as_slice())
    }

    pub fn links(&self) -> JSlice {
        JSlice::from_slice(self.graph.links.as_slice())
    }

    pub fn get_path(&self, i: usize) -> *const Path {
        &self.graph.paths[i]
    }

    pub fn path_count(&self) -> usize {
        self.graph.paths.len()
    }

    pub fn get_segment(&self, i: usize) -> *const Segment {
        &self.graph.segments[i]
    }

    pub fn segment_count(&self) -> usize {
        self.graph.segments.len()
    }
}

#[wasm_bindgen]
pub async fn fetch_gfa(url: JsString) -> Result<WrappedGFA, JsValue> {
    let url: String = url.as_string().ok_or("Error parsing url")?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text: JsValue = JsFuture::from(resp.text()?).await?;

    let wrapped_gfa = WrappedGFA::parse(text.dyn_into().unwrap());

    Ok(wrapped_gfa)
}

/////////////////
///////////////// JSON-serializable GFA types
/////////////////

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JSegment {
    pub name: String,
    pub sequence: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JLink {
    pub from_segment: String,
    pub from_orient: bool,
    pub to_segment: String,
    pub to_orient: bool,
    pub overlap: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JPath {
    pub path_name: String,
    pub segment_names: Vec<(String, bool)>,
    pub overlaps: Vec<String>,
}

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

fn parse_line_util(bs: &[u8]) -> Option<Line<BString, ()>> {
    let parser: GFAParser<()> = GFAParser::new();
    parser.parse_line(bs)
}

#[wasm_bindgen]
pub fn parse_line(s: &str) -> JsValue {
    let bs = s.as_bytes();
    let line = parse_line_util(bs);
    let jline = line.and_then(line_to_jline);
    JsValue::from_serde(&jline).unwrap()
}

#[wasm_bindgen]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct WGFA {
    segments: Vec<JSegment>,
    links: Vec<JLink>,
    paths: Vec<JPath>,
}

impl WGFA {
    fn parse_gfa(gfa_string: JsString) -> WGFA {
        let mut wgfa: WGFA = Default::default();

        gfa_string
            .split("\n")
            .iter()
            .filter_map(|l| {
                let line = l.as_string()?;
                let bs = line.as_bytes();
                let parsed = parse_line_util(bs)?;
                line_to_jline(parsed)
            })
            .for_each(|jl| match jl {
                JLine::Segment(js) => wgfa.segments.push(js),
                JLine::Link(js) => wgfa.links.push(js),
                JLine::Path(js) => wgfa.paths.push(js),
            });

        wgfa
    }
}

#[wasm_bindgen]
pub struct StrPtr {
    pub start: *const u8,
    pub len: usize,
}

impl StrPtr {
    pub fn new(s: &str) -> StrPtr {
        let start = s.as_ptr();
        let len = s.len();
        StrPtr { start, len }
    }
}

#[wasm_bindgen]
impl WGFA {
    pub fn segments(&self) -> *const JSegment {
        self.segments.as_slice().as_ptr()
    }

    // pub fn get_segment(&self, ix: usize) -> bool {
    //     let r = &self.segments[ix];
    //     r.as_ptr()
    // }

    pub fn get_segment_name(&self, ix: usize) -> StrPtr {
        StrPtr::new(&self.segments[ix].name)
    }

    pub fn get_segment_seq(&self, ix: usize) -> StrPtr {
        StrPtr::new(&self.segments[ix].sequence)
    }
}

#[wasm_bindgen]
pub fn seg_size() -> usize {
    std::mem::size_of::<JSegment>()
}

#[wasm_bindgen]
pub fn string_size() -> usize {
    std::mem::size_of::<String>()
}

#[wasm_bindgen]
pub async fn fetch_wgfa(url: JsString) -> Result<WGFA, JsValue> {
    let url: String = url.as_string().ok_or("Error parsing url")?;

    let mut opts = RequestInit::new();
    opts.method("GET");
    // opts.mode(RequestMode::Cors);
    opts.mode(RequestMode::SameOrigin);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text: JsValue = JsFuture::from(resp.text()?).await?;

    let string: JsString = text.dyn_into().unwrap();

    let wgfa = WGFA::parse_gfa(string);

    Ok(wgfa)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JLine {
    Segment(JSegment),
    Link(JLink),
    Path(JPath),
}

fn orient_bool(o: Orientation) -> bool {
    match o {
        Orientation::Forward => true,
        Orientation::Backward => false,
    }
}

fn line_to_jline(line: Line<BString, ()>) -> Option<JLine> {
    match line {
        Line::Segment(seg) => {
            let jseg = JSegment {
                name: seg.name.to_string(),
                sequence: seg.sequence.to_string(),
            };
            Some(JLine::Segment(jseg))
        }
        Line::Link(link) => {
            let jlink = JLink {
                from_segment: link.from_segment.to_string(),
                from_orient: orient_bool(link.from_orient),
                to_segment: link.to_segment.to_string(),
                to_orient: orient_bool(link.to_orient),
                overlap: link.overlap.to_string(),
            };

            Some(JLine::Link(jlink))
        }
        Line::Path(path) => {
            let path_name = path.path_name.to_string();
            let segment_names = path
                .iter()
                .map(|(n, o)| (n.to_string(), orient_bool(o)))
                .collect();
            let overlaps = path.overlaps.iter().map(|bs| bs.to_string()).collect();

            let jpath = JPath {
                path_name,
                segment_names,
                overlaps,
            };

            Some(JLine::Path(jpath))
        }
        _ => None,
    }
}
