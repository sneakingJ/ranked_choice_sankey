use std::cell::RefCell;
use std::rc::Rc;
use crate::node::Node;

trait FlowConstants {
    const FLOW_COLOR_WIN: &'static str = "#777879";
    const FLOW_COLOR_LOSE: &'static str = "#484848";
}

#[derive(Clone)]
pub struct Flow {
    origin: Rc<RefCell<Node>>,
    destination: Rc<RefCell<Node>>,
    size: i32,
    color: String
}

impl FlowConstants for Flow {}

impl Flow {
    pub fn new(origin: Rc<RefCell<Node>>, destination: Rc<RefCell<Node>>, size: i32) -> Self {
        Self {
            origin,
            destination,
            size,
            color: Self::FLOW_COLOR_LOSE.to_string()
        }
    }
    
    pub fn origin(&self) -> &Rc<RefCell<Node>> {
        &self.origin
    }

    pub fn destination(&self) -> &Rc<RefCell<Node>> {
        &self.destination
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn set_winning_color(&mut self) {
        self.color = Self::FLOW_COLOR_WIN.to_string();
    }

    pub fn set_origin(&mut self, origin: Rc<RefCell<Node>>) {
        self.origin = origin;
    }

    pub fn set_destination(&mut self, destination: Rc<RefCell<Node>>) {
        self.destination = destination;
    }
}