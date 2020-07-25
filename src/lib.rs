mod utils;

use wasm_bindgen::prelude::*;

use bstr::{BStr, BString};

use gfa::{
    gfa::{Link, Path, Segment, GFA},
    optfields::OptFields,
    parser::GFAParser,
};

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
