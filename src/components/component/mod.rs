use super::{Color, Size};
//use render::utils::Rect;
use glium::Frame;

mod rectangle;

pub use self::rectangle::Rectangle;

pub trait Component : ::std::fmt::Debug {
    fn get_children<'a>(&'a self) -> &'a Vec<&'a Component>;
    fn render(&self, frame: &mut Frame);
}

/*
pub fn calculate_size_from_parent(
    parent: Rect,
    left: Size,
    top: Size,
    right: Size,
    bottom: Size,
    width: Size,
    height: Size
) -> Rect {
    let left = {
        if left.is_known() {
            left.get_pixel_position(parent.left(), parent.right())
        } else if right.is_known() && width.is_known() {
            let right = right.get_pixel_position_reversed(parent.left(), parent.right());
            let width = width.get_pixel_position(parent.left(), right);
            right - width
        } else {
            parent.left()
        }
    };

    let top = {
        if top.is_known() {
            top.get_pixel_position(parent.top(), parent.bottom())
        } else if bottom.is_known() && height.is_known() {
            let bottom = bottom.get_pixel_position_reversed(parent.top(), parent.bottom());
            let height = width.get_pixel_position(parent.top(), bottom);
            bottom - height
        } else {
            parent.top()
        }
    };

    let width = {
        if width.is_known() {
            width.get_pixel_position(left, parent.right())
        } else if right.is_known() {
            let right = right.get_pixel_position_reversed(left, parent.right());
            right - left
        } else {
            parent.right() - left
        }
    };

    let height = {
        if height.is_known() {
            height.get_pixel_position(top, parent.bottom())
        } else if bottom.is_known() {
            let bottom = bottom.get_pixel_position_reversed(top, parent.bottom());
            bottom - top
        } else {
            parent.bottom() - top
        }
    };

    Rect {
        x: left,
        y: top,
        width: width,
        height: height,

        screen_width: parent.screen_width,
        screen_height: parent.screen_height,
    }
}*/
/*
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
*/