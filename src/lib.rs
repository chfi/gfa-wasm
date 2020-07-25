mod utils;

use js_sys::{Array, JsString};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use bstr::BString;

use gfa::{
    gfa::{Line, Orientation},
    parser::GFAParser,
};

macro_rules! log {
    ( $( $t:tt )*) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize, Deserialize)]
pub struct JSegment {
    pub name: String,
    pub sequence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JLink {
    pub from_segment: String,
    pub from_orient: bool,
    pub to_segment: String,
    pub to_orient: bool,
    pub overlap: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JPath {
    pub path_name: String,
    pub segment_names: Vec<(String, bool)>,
    pub overlaps: Vec<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct JGFA {
    pub segments: Vec<JSegment>,
    pub links: Vec<JLink>,
    pub paths: Vec<JPath>,
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
            let name = seg.name.to_string();
            let sequence = seg.sequence.to_string();
            let jseg = JSegment { name, sequence };
            Some(JLine::Segment(jseg))
        }
        Line::Link(link) => {
            let from_segment = link.from_segment.to_string();
            let from_orient = orient_bool(link.from_orient);
            let to_segment = link.to_segment.to_string();
            let to_orient = orient_bool(link.to_orient);
            let overlap = link.overlap.to_string();
            let jlink = JLink {
                from_segment,
                from_orient,
                to_segment,
                to_orient,
                overlap,
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

#[wasm_bindgen]
pub async fn get_lil_json() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    // opts.mode(RequestMode::Cors);
    opts.mode(RequestMode::SameOrigin);

    let url = "./lil.gfa";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text: JsValue = JsFuture::from(resp.text()?).await?;

    let string: JsString = text.dyn_into().unwrap();

    let jgfa = parse_gfa(string);

    Ok(JsValue::from_serde(&jgfa).unwrap())
}

fn parse_gfa(s: JsString) -> JGFA {
    let mut jgfa: JGFA = Default::default();
    s.split("\n")
        .iter()
        .filter_map(|l| {
            let line = l.as_string()?;
            let bs = line.as_bytes();
            let parsed = parse_line_util(bs)?;
            line_to_jline(parsed)
        })
        .for_each(|jl| match jl {
            JLine::Segment(js) => jgfa.segments.push(js),
            JLine::Link(js) => jgfa.links.push(js),
            JLine::Path(js) => jgfa.paths.push(js),
        });

    jgfa
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
