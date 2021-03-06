#![allow(dead_code, unused_imports, unused_variables)]

#[macro_use]
extern crate gui;

use gui::components::{Color, Size, Rectangle};
use gui::render::Render;

pub fn main() {
    let color_blue = Color::RGB(0f32, 1f32, 0f32);
    let margin = Size::Pixels(50);

    let mut view = layout!(Rectangle {
        // special layout syntax
        left: 50 px,
        top: 50 px,
        right: 50 px,
        bottom: 50 px,
        color: rgb(1, 0, 0),

        Rectangle {
            // variables work as well
            left: margin,
            right: margin,
            top: margin,
            bottom: margin,
            color: color_blue,

            Rectangle {
                // Or direct values from rust
                left: Size::Pixels(50),
                right: Size::Pixels(50),
                top: Size::Pixels(50),
                bottom: Size::Pixels(50),
                color: Color::RGB(0f32, 0f32, 1f32),
            }
        }
    });
/*
    let mut render = Render::new();
    render.mount(&mut view);

    render.render().unwrap();*/
}
