#[macro_use]
extern crate gui;

// use gui::components::{ ClickEvent, LoadEvent, View, PredefinedColor };
use gui::render::Render;

pub fn main() {
    let mut view = layout!(Component {
        left: 50 px,
        top: 50 px,
        right: 50 px,
        bottom: 50 px,
        color: red,
        //onLoad: onload_callback,

        Component {
            left: 50 px,
            right: 50 px,
            top: 50 px,
            bottom: 50 px,
            color: green,
            //onClick: click_callback,

            Component {
                left: 50 px,
                right: 50 px,
                top: 50 px,
                bottom: 50 px,
                color: blue,
                //onClick: click_callback
            }
        }
    });

    let mut render = Render::new();
    render.mount(&mut view);

    render.render().unwrap();
}

// fn onload_callback(view: &mut View, id: u64, _event: &mut LoadEvent) {
// view.get_component_mut(id).unwrap().color = PredefinedColor::green.into();
// }
//
// fn click_callback(view: &mut View, id: u64, _event: &mut ClickEvent) {
// let target = view.get_component(id).unwrap().clone();
// println!("Click on {:?}", target);
// if let Some(mut parent) = view.get_parent_mut(&target) {
// println!("Parent color: {:?}", parent.color);
// }
// }
//
