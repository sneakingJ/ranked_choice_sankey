use std::rc::Rc;
use wasm_bindgen::JsValue;
use crate::flow::Flow;
use crate::canvas::Canvas;
use crate::colors::Colors;
use crate::config::Config;
use crate::node::Node;

pub struct Voting {
    canvas: Canvas,
    nodes_per_round: Vec<Vec<Rc<Node>>>,
    flows: Vec<Flow>
}

impl Voting {    
    pub fn new(canvas_id: &str, js_config: JsValue) -> Self {
        let canvas = Canvas::new(canvas_id);
        
        let mut colors = Colors::default();
        
        let config = Config::new();
        
        let (nodes_per_round, flows) = config.process(js_config, &mut colors);
        
        Self {
            canvas,
            nodes_per_round,
            flows
        }
    }
    
    pub fn exec(&mut self) {
        self.canvas.process_nodes(&self.nodes_per_round);

        self.canvas.process_flows(&self.flows);
        
        self.canvas.draw();
    }
}