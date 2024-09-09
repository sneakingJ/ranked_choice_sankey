use std::cell::RefCell;
use std::rc::Rc;
use crate::node::Node;

pub struct Flow {
    origin: Rc<RefCell<Node>>,
    destination: Rc<RefCell<Node>>,
    size: i32
}

impl Flow {
    pub fn new(origin: Rc<RefCell<Node>>, destination: Rc<RefCell<Node>>, size: i32) -> Self {
        Self {
            origin,
            destination,
            size
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
}