use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::node::Node;
use crate::flow::Flow;

trait CanvasConstants {
    const REC_WIDTH: usize = 20;
    const WINNER_REC_HEIGHT_PERCENT: f64 = 3.0;
}

pub struct Canvas {
    width: u32,
    height: u32,
    context: CanvasRenderingContext2d,
    winner_votes: u32,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
    flows: Vec<Flow>
}

impl CanvasConstants for Canvas {}

impl Canvas {
    pub fn new(id: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let parent = canvas.parent_element().unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let width = parent.client_width().abs() as u32;
        let height = parent.client_height().abs() as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        Self {
            width,
            height,
            context,
            nodes: HashMap::new(),
            winner_votes: 0,
            flows: vec![]
        }
    }

    pub fn process_nodes(&mut self, config: &Vec<Vec<Rc<RefCell<Node>>>>) {
        let last_round = &config[config.len() - 1];
        self.winner_votes = last_round[0].borrow().votes();

        let round_amount = config.len();

        let space_between_y = self.calc_space_between_y(&config[0]);
        
        for (round_index, nodes) in config.iter().enumerate() {
            let offset_x = self.calc_offset_x(round_amount, round_index);

            let mut offset_y = self.calc_round_height(nodes, space_between_y);

            for node in nodes.iter() {
                let mut borrowed_node = node.borrow_mut();
                
                let rec_height = self.calc_single_rec_height(borrowed_node.votes());
                let name = borrowed_node.name().to_string();

                // Never show labels in second round but always in first
                match round_index {
                    0 => borrowed_node.set_label(name),
                    1 => borrowed_node.set_label("".to_string()),
                    _ => ()
                };
                
                borrowed_node.set_x_pos(offset_x as f64);
                borrowed_node.set_y_pos(offset_y as f64);
                borrowed_node.set_height(rec_height as f64);

                self.nodes.insert(borrowed_node.id().to_string(), Rc::clone(node));

                offset_y += rec_height + space_between_y;
            }
        }
    }

    pub fn process_flows(&mut self, flows: &Vec<Flow>) {
        for flow in flows.iter() {
            let Some(origin) = self.nodes.get(&flow.origin().borrow().id().to_string()) else { continue; };
            let Some(destination) = self.nodes.get(&flow.destination().borrow().id().to_string()) else { continue; };

            let mut new_flow = flow.clone();
            new_flow.set_origin(Rc::clone(origin));
            new_flow.set_destination(Rc::clone(destination));
            
            self.flows.push(new_flow);
        }
    }

    pub fn draw(&self) {
        let rec_width = Self::REC_WIDTH as f64;

        for (_, node) in self.nodes.iter() {
            let node_borrowed = node.borrow();

            self.draw_rec(
                node_borrowed.x_pos(),
                node_borrowed.y_pos(),
                rec_width,
                node_borrowed.height(),
                &node_borrowed.color(),
                node_borrowed.votes(),
                &node_borrowed.label()
            );
        }

        let _ = self.context.set_global_composite_operation("screen");

        let mut origin_y_offsets: HashMap<String, f64> = HashMap::new();
        let mut destination_y_offsets: HashMap<String, f64> = HashMap::new();

        for flow in &self.flows {
            self.context.set_fill_style(&JsValue::from_str(flow.color().as_ref()));
            
            let origin = flow.origin().borrow();
            let destination = flow.destination().borrow();

            let origin_percentage: f64 = flow.size() as f64 / origin.votes() as f64;
            let origin_height = origin.height() * origin_percentage;
            let destination_percentage: f64 = flow.size() as f64 / destination.votes() as f64;
            let destination_height = destination.height() * destination_percentage;

            let origin_y_offset = origin_y_offsets.entry(origin.id().to_string()).or_insert(0.0);
            let destination_y_offset = destination_y_offsets.entry(destination.id().to_string()).or_insert(0.0);

            self.draw_flow(
                origin.x_pos() + rec_width,
                origin.y_pos() + *origin_y_offset,
                destination.x_pos(),
                destination.y_pos() + *destination_y_offset,
                origin_height,
                destination_height
            );

            *origin_y_offset += origin_height;
            *destination_y_offset += destination_height;
        }
    }

    fn draw_rec(&self, pos_x: f64, pos_y: f64, width: f64, height: f64, color: &str, votes: u32, label: &str) {
        self.context.set_font("bold 16px sans-serif");

        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(pos_x, pos_y, width, height);

        self.context.set_fill_style(&JsValue::from_str("#000000"));
        self.draw_votecount(pos_x, pos_y, height, votes);

        self.context.set_fill_style(&JsValue::from_str("#FFFFFF"));
        self.draw_label(pos_x, pos_y, height, label);
    }

    fn draw_votecount(&self, pos_x: f64, pos_y: f64, height: f64, votes: u32) {
        self.context.save();
        let _ = self.context.translate(pos_x + Self::REC_WIDTH as f64 / 2.0, pos_y + height / 2.0);
        let _ = self.context.rotate(-90.0 * std::f64::consts::PI / 180.0);

        let text = format!("{}", votes);
        let text_width = self.context.measure_text(&text).unwrap().width();
        let text_height = 12.0;
        self.context.fill_text(
            &text,
            -text_width / 2.0,
            text_height / 2.0
        ).unwrap();

        self.context.restore();
    }

    fn draw_label(&self, pos_x: f64, pos_y: f64, height: f64, label: &str) {
        let vertical_center = pos_y + height / 2.0;
        let text_height = 12.0;

        // Label first round to the right, other rounds to the left of node
        let text_width = self.context.measure_text(label).unwrap().width();
        let label_pos_x = if pos_x - text_width <= 0.0  {
            pos_x + Self::REC_WIDTH as f64 + 10.0
        } else {
            pos_x - text_width - 10.0
        };

        self.context.fill_text(
            label,
            label_pos_x,
            vertical_center + text_height / 2.0
        ).unwrap();
    }

    fn draw_flow(&self, start_x: f64, start_y: f64, end_x: f64, end_y: f64, height_origin: f64, height_destination: f64) {
        let control_point_1_x = start_x + (end_x - start_x) * 0.5;
        let control_point_1_y = start_y;

        let control_point_2_x = end_x + (start_x - end_x) * 0.5;
        let control_point_2_y = end_y;

        self.context.begin_path();

        self.context.move_to(start_x, start_y);
        self.context.bezier_curve_to(
            control_point_1_x, control_point_1_y,
            control_point_2_x, control_point_2_y,
            end_x, end_y,
        );

        self.context.line_to(end_x, end_y + height_destination);
        self.context.bezier_curve_to(
            control_point_2_x, control_point_2_y + height_destination,
            control_point_1_x, control_point_1_y + height_origin,
            start_x, start_y + height_origin,
        );

        self.context.fill();
    }

    fn calc_winner_rec_height(&self) -> f64 {
        self.height as f64 / Self::WINNER_REC_HEIGHT_PERCENT
    }

    fn calc_single_rec_height(&self, amount_votes: u32) -> u32 {
        let percentage: f64 = amount_votes as f64 / self.winner_votes as f64;

        let height = self.calc_winner_rec_height() * percentage;

        if height < 16.0 {
            return 16;
        }

        height as u32
    }

    fn calc_offset_x(&self, round_amount: usize, round_index: usize) -> usize {
        if round_index == 0 {
            return 0;
        }

        (self.width as usize * round_index) / (round_amount - 1) - Self::REC_WIDTH
    }

    fn calc_space_between_y(&self, entries: &[Rc<RefCell<Node>>]) -> u32 {
        if entries.len() == 1 {
            return 0;
        }

        let space_left = self.height - self.calc_all_recs_height(entries);

        space_left / (entries.len() as u32 - 1)
    }

    fn calc_all_recs_height(&self, entries: &[Rc<RefCell<Node>>]) -> u32 {
        entries.iter().fold(0, |acc, x| acc + self.calc_single_rec_height(x.borrow().votes()))
    }

    fn calc_round_height(&self, entries: &[Rc<RefCell<Node>>], calc_space_between_y: u32) -> u32 {
        let total_height = self.calc_all_recs_height(entries) + (calc_space_between_y * (entries.len() as u32 - 1));
        let space_left = self.height - total_height;

        space_left / 2
    }
}