#![allow(unused_variables, unused_mut, dead_code)]

#[macro_use]
mod macros;

pub mod component_state;
pub mod component;
pub mod node;
pub mod states;
pub mod error;
pub mod point;
pub mod rectangle;
pub mod canvas;
pub mod canvas_components;

pub mod prelude {
    pub use component::Component;
    pub use states::{Click, Render};
    pub use error::Result;
}
