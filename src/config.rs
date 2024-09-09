use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use crate::colors::Colors;
use crate::node::Node;
use crate::flow::Flow;

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(&self, js_config: JsValue, colors: &mut Colors) -> (Vec<Vec<Rc<RefCell<Node>>>>, Vec<Flow>) {
        let config: Vec<Vec<String>> = serde_wasm_bindgen::from_value(js_config).unwrap();

        let mut nodes_per_round: Vec<Vec<Rc<RefCell<Node>>>> = vec![];
        let mut flows: Vec<Flow> = vec![];

        let mut current_round: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
        let mut last_round: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();

        let mut round_index = 0;

        for flow in config.iter().rev() {
            let Some((origin_label, destination_label, size)) = self.destructure_flow(flow) else {
                continue;
            };

            // Last round has no origin
            if nodes_per_round.is_empty() {
                let last_round_node = self.create_rc_node(destination_label, round_index, colors);
                last_round.insert(destination_label, Rc::clone(&last_round_node));
                nodes_per_round.push(vec![Rc::clone(&last_round_node)]);
                round_index += 1;
            }

            // Some flows have a size of 0. We only need it when the node doesn't already exist
            if size < 1 && current_round.contains_key(&destination_label) {
                continue;
            }

            // A round ends as soon as we find a flow where the destination is one of the current round origins
            if current_round.contains_key(&destination_label) {
                nodes_per_round.push(self.sort_current_round(&current_round));

                last_round = current_round.clone();
                current_round.clear();

                round_index += 1;
            }

            let origin = self.create_rc_node(origin_label, round_index, colors);

            if !current_round.contains_key(&origin_label) {
                current_round.insert(origin_label, Rc::clone(&origin));
            }

            let Some(destination) = last_round.get(destination_label) else { continue; };

            let mut new_flow = Flow::new(Rc::clone(&origin), Rc::clone(&destination), size);

            // Have a darker color for flows from a losing game and only show labels for losers
            if origin.borrow().name() == destination.borrow().name() {
                origin.borrow_mut().set_label("".to_string());
                new_flow.set_winning_color();
            }

            flows.push(new_flow);
        }

        nodes_per_round.push(self.sort_current_round(&current_round));

        nodes_per_round.reverse();

        (nodes_per_round, flows)
    }

    fn destructure_flow<'b>(&self, flow: &'b [String]) -> Option<(&'b str, &'b str, i32)> {
        let mut flow_iter = flow.iter();
        let Some(origin_label) = flow_iter.next() else { return None; };
        let Some(destination_label) = flow_iter.next() else { return None; };
        let Some(size) = flow_iter.next() else { return None; };
        let Ok(size) = size.parse::<i32>() else { return None; };

        Some((origin_label.trim(), destination_label.trim(), size))
    }

    fn create_rc_node(&self, label: &str, round_index: usize, colors: &mut Colors) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::new(label, colors, round_index)))
    }

    fn sort_current_round(&self, nodes: &HashMap<&str, Rc<RefCell<Node>>>) -> Vec<Rc<RefCell<Node>>> {
        let mut sorted_nodes: Vec<(String, Rc<RefCell<Node>>)> = vec![];

        for node in nodes.iter() {
            sorted_nodes.push((node.0.to_string(), Rc::clone(node.1)));
        }

        sorted_nodes.sort_by(|a, b| b.1.borrow().votes().cmp(&a.1.borrow().votes()));

        sorted_nodes.into_iter().map(|(_, node)| node).collect()
    }
}