use super::{Color, Component, Size};
use glium::Frame;

#[derive(Debug)]
pub struct Rectangle<'a> {
    pub parent: &'a Component,
    pub children: Vec<&'a Component>,

    pub left: Size,
    pub top: Size,
    pub right: Size,
    pub bottom: Size,

    pub width: Size,
    pub height: Size,
    pub color: Color,
}

impl<'a> Rectangle<'a> {
    pub fn from_parent(parent: &'a Component) -> Rectangle<'a> {
        Rectangle {
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

impl<'a> Component for Rectangle<'a> {
    fn get_children<'b>(&'b self) -> &Vec<&'b Component> {
        &self.children
    }

    fn render(&self, _frame: &mut Frame) {
        unimplemented!()
    }
}
