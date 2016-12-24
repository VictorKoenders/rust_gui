#[macro_use] extern crate gui;

use gui::components::{ ClickEvent, LoadEvent, View, PredefinedColor };
use gui::render::Render;

pub fn main(){
  let mut view = layout!(Component { left: 10 px, top: 10 px, width: 50%, bottom: 10 px, color: red });
  /*layout!(Component {
    left: 10 px
    top: 10 px
    right: 10 px
    bottom: 10 px
    color: red

    onLoad: onload_callback
    Component {
      width: 50%
      height: 50 px
      color: blue
      onClick: click_callback
    }
  });*/

  let mut render = Render::new();
  render.mount(&mut view);

  render.render();
}

fn onload_callback(view: &mut View, id: u64, _event: &mut LoadEvent) {
  view.get_component_mut(id).unwrap().color = PredefinedColor::green.into();
}

fn click_callback(view: &mut View, id: u64, _event: &mut ClickEvent) {
  let target = view.get_component(id).unwrap().clone();
  println!("Click on {:?}", target);
  if let Some(mut parent) = view.get_parent_mut(&target) {
    print!("Parent color: {:?}", parent.color);
    parent.color = PredefinedColor::blue.into();
    println!(" -> {:?}", parent.color);
  }
}
