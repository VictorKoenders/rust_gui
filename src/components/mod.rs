mod component;
mod view;
mod utils;
mod events;

pub use self::component::{Component, Rectangle};
pub use self::utils::*;
pub use self::view::*;
pub use self::events::*;

#[macro_export]
macro_rules! layout {
  ($component:tt { $($inner:tt)* }) => {{
    let mut container = $crate::components::Container::new();
    let mut component = $crate::components::$component::from_parent(&container.view);
    layout_inner!(container, component, $($inner)*);

    container
  }}
}

#[macro_export]
macro_rules! layout_inner {
// Inner component
//
// Component {
//   ...
// }
  ($view:tt, $parent_component:tt, $name:tt { $($inner:tt)* } $($remaining:tt)*) => {
    let mut inner_component = $crate::components::$name::from_parent(&$parent_component);

    layout_inner!($view, inner_component, $($inner)*);
    $parent_component.children.push(&inner_component);

    $view.view.add_component(Box::new(inner_component));

    layout_inner!($view, $parent_component, $($remaining)*);
  };

// Left with pixel value
//
// left: 100 px
  ($view:tt, $component:expr, left: $value:tt px $($remaining:tt)*) => {
    $component.left = $crate::components::Size::Pixels($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Left with variable value
//
// let left_offset = Size::Pixels(10);
// left: left_offset
  ($view:tt, $component:expr, left: $value:expr, $($remaining:tt)*) => {
    $component.left = ::std::convert::From::from($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Top with pixel value
//
// top: 100 px
  ($view:tt, $component:expr, top: $value:tt px $($remaining:tt)*) => {
    $component.top = $crate::components::Size::Pixels($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Top with variable value
//
// let top_offset = Size::Pixels(10);
// top: top_offset
  ($view:tt, $component:expr, top: $value:expr, $($remaining:tt)*) => {
    $component.top = ::std::convert::From::from($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Right with pixel value
//
// right: 100 px
  ($view:tt, $component:expr, right: $value:tt px $($remaining:tt)*) => {
    $component.right = $crate::components::Size::Pixels($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Right with variable value
//
// let right_offset = Size::Pixels(10);
// right: right_offset
  ($view:tt, $component:expr, right: $value:expr, $($remaining:tt)*) => {
    $component.right = ::std::convert::From::from($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Bottom with pixel value
//
// bottom: 100 px
  ($view:tt, $component:expr, bottom: $value:tt px $($remaining:tt)*) => {
    $component.bottom = $crate::components::Size::Pixels($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Bottom with pixel value
//
// let bottom_offset = Size::Pixels(10);
// bottom: bottom_offset
  ($view:tt, $component:expr, bottom: $value:expr, $($remaining:tt)*) => {
    $component.bottom = $value;
    layout_inner!($view, $component, $($remaining)*);
  };

// Width with % value
//
// width: 100%
  ($view:tt, $component:expr, width: $value:tt % $($remaining:tt)*) => {
    $component.width = $crate::components::Size::Percentage($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Width with pixel value
//
// width: 100 px
  ($view:tt, $component:expr, width: $value:tt px $($remaining:tt)*) => {
    $component.width = $crate::components::Size::Pixels($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Height with % value
//
// height: 100%
  ($view:tt, $component:expr, height: $value:tt % $($remaining:tt)*) => {
    $component.height = $crate::components::Size::Percentage($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Height with pixel value
//
// height: 100 px
  ($view:tt, $component:expr, height: $value:tt px $($remaining:tt)*) => {
    $component.height = $crate::components::Size::Pixels($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// Color with RGB value
//
// color: "red"
  ($view:tt, $component:expr, color: rgb($r:tt, $g:tt, $b:tt) $($remaining:tt)*) => {
    $component.color = $crate::components::Color::RGB($r as f32, $g as f32, $b as f32);
    layout_inner!($view, $component, $($remaining)*);
  };

// Color with argument value
//
// let color_red = Color::RGB(1f32, 1f32, 1f32);
// color: color_red
  ($view:tt, $component:expr, color: $value:expr, $($remaining:tt)*) => {
    $component.color = ::std::convert::From::from($value);
    layout_inner!($view, $component, $($remaining)*);
  };

// OnClick event handler
//
// onClick: some_function
  ($view:tt, $component:expr, onClick: $value:tt $($remaining:tt)*) => {
    $view.events.click
      .entry($component)
      .or_insert_with(||Vec::new())
      .push(Box::new($value));
    layout_inner!($view, $component, $($remaining)*);
  };

// OnLoad event handler
//
// onLoad: some_function
  ($view:tt, $component:expr, onLoad: $value:tt $($remaining:tt)*) => {
    $view.events.load
      .entry($component)
      .or_insert_with(||Vec::new())
      .push(Box::new($value));
    layout_inner!($view, $component, $($remaining)*);
  };

// Strip out extra commas between tokens
  ($view:tt, $parent_component:tt, , $($remaining:tt)*) => {
    layout_inner!($view, $parent_component, $($remaining)*);
  };

// Strip out extra semicolons between tokens
  ($view:tt, $parent_component:tt, ; $($remaining:tt)*) => {
    layout_inner!($view, $parent_component, $($remaining)*);
  };

// Captures empty entries after Component { }
  ($view:tt, $component:tt, ) => {};
}
