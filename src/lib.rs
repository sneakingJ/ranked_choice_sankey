use wasm_bindgen::prelude::*;

use crate::voting::Voting;

mod canvas;
mod colors;
mod config;
mod flow;
mod node;
mod voting;

#[wasm_bindgen]
pub fn start(canvas_id: Option<String>, js_config: JsValue) {
    let mut voting = Voting::new(&canvas_id.unwrap(), js_config);

    voting.exec();
}
