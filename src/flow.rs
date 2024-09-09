use std::rc::Rc;
use crate::node::Node;

pub struct Flow {
    origin: Rc<Node>,
    destination: Rc<Node>,
    size: i32
}

impl Flow {
    pub fn new(origin: Rc<Node>, destination: Rc<Node>, size: i32) -> Self {
        Self {
            origin,
            destination,
            size
        }
    }
    
    pub fn origin(&self) -> &Rc<Node> {
        &self.origin
    }

    pub fn destination(&self) -> &Rc<Node> {
        &self.destination
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}