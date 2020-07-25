mod utils;

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;

use bstr::{BStr, BString};

use gfa::{
    gfa::{Line, Link, Orientation, Path, Segment, GFA},
    optfields::OptFields,
    parser::GFAParser,
};

use lazy_static::lazy_static;

// pub struct

#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-gfa!");
}

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
