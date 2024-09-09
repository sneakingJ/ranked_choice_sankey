mod canvas;
mod node;
mod flow;
mod voting;
mod config;
mod colors;

use wasm_bindgen::prelude::*;
use crate::voting::Voting;

#[wasm_bindgen]
pub fn start(canvas_id: Option<String>, js_config: JsValue) {
    let mut voting = Voting::new(&canvas_id.unwrap(), js_config);

    voting.exec();
}