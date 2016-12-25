use glium::glutin::{ElementState, Event, VirtualKeyCode};
use components::{Component, Container};
use vecmath::Vector2;

mod render_target;
mod error;
mod utils;

pub type Result<T> = ::std::result::Result<T, error::RenderError>;

#[derive(Default)]
pub struct Render<'a> {
    pub container: Option<&'a mut Container>,
}


impl<'a> Render<'a> {
    pub fn new() -> Render<'a> {
        Render { container: None }
    }

    pub fn mount(&mut self, container: &'a mut Container) {
        self.container = Some(container);
    }


    fn get_component_by_id(&self, id: u64) -> Option<Component> {
        match self.container {
            Some(ref c) => c.view.get_component(id).cloned(),
            None => None,
        }
    }

    fn get_root_component_id(&self) -> Option<u64> {
        match self.container {
            Some(ref c) => c.view.get_root_id(),
            None => None,
        }
    }

    pub fn render(mut self) -> Result<()> {
        if self.container.is_none() {
            panic!("Cannot start render without view");
        }
        let mut target = render_target::RenderTarget::new()?;

        'render_loop: loop {
            target.render(&mut self)?;

            for ev in target.display.poll_events() {
                match ev {
                    Event::Closed |
                    Event::KeyboardInput(ElementState::Pressed,
                                         _,
                                         Some(VirtualKeyCode::Escape)) => break 'render_loop,
                    _ => (),
                }
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: Vector2<f32>,
    pub tex_coords: Vector2<f32>,
}
implement_vertex!(Vertex2D, position, tex_coords);
