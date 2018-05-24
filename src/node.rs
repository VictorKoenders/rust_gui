use std::rc::{Weak, Rc};
use std::cell::RefCell;
use component_state::ComponentState;
use component::Component;

pub struct Node {
    pub state: ComponentState,
    pub component: Box<Component>,
    pub parent: Weak<RefCell<Node>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

