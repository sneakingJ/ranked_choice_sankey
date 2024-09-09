mod canvas;
mod node;
mod flow;
mod voting;
mod config;
mod colors;

use wasm_bindgen::prelude::*;
use crate::voting::Voting;

#[wasm_bindgen]
pub fn start(js_config: JsValue) {
    let canvas_id = "canvas";

    let mut voting = Voting::new(canvas_id, js_config);

    voting.exec();
}