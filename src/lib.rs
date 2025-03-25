// lib.rs

use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn run() {
    // Get the document and append a "Hello WASM!" paragraph element
    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let para = document.create_element("h1").unwrap();
    para.set_inner_html("Hello, Web Assembly!");
    body.append_child(&para).unwrap();
}

