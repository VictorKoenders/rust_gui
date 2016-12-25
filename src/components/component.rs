use super::{Color, Size};

static mut NEXT_ID: u64 = 1;
pub const COMPONENT_NONE: u64 = 0;

// TODO: Make this a trait and make a panel to implement this
// TODO: Make a ComponentWrapper to cache render data
#[derive(Debug, Clone)]
pub struct Component {
    pub id: u64,

    pub parent: u64,
    pub children: Vec<u64>,

    pub left: Size,
    pub top: Size,
    pub right: Size,
    pub bottom: Size,

    pub width: Size,
    pub height: Size,
    pub color: Color,
}

impl Component {
    pub fn new(parent: u64) -> Component {
        Component {
            id: unsafe {
                let id = NEXT_ID;
                NEXT_ID += 1;
                id
            },
            parent: parent,
            children: Vec::new(),
            left: Size::Unknown,
            top: Size::Unknown,
            right: Size::Unknown,
            bottom: Size::Unknown,
            width: Size::Unknown,
            height: Size::Unknown,
            color: Color::Unknown,
        }
    }
}
