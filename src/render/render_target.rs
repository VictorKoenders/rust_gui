use glium::{DisplayBuild, IndexBuffer, Frame, VertexBuffer, Program, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use super::{Render, Result, Vertex2D};
use components::{Component, SizeType};
use glium::glutin::WindowBuilder;
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use super::utils::Rect;

macro_rules! try_get {
  ($stmt:expr, $($error:tt)*) => {
    match $stmt {
      Some(c) => c,
      None => return Err($crate::render::error::RenderError::$($error)*)
    }
  }
}

pub struct RenderTarget {
    pub display: GlutinFacade,
    pub program: Program,
    pub texture: Texture2d,
}

impl RenderTarget {
    pub fn new() -> Result<RenderTarget> {
        let display = WindowBuilder::new().with_vsync().build_glium()?;
        let texture = Texture2d::new(&display, vec![vec![0xFF0000FFu32]])?;

        // TODO: Get rid of texture and just pass a color to the shader
        // TODO: Maybe make components render themselves so we can make custom components?
        let program = Program::from_source(&display,
                                           r#"#version 140
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

void main() {
	v_tex_coords = tex_coords;
	gl_Position = vec4(position, 0.0, 1.0);
}
"#,
                                           r#"#version 140
uniform vec4 component_color;
out vec4 color;

void main() {
	color = component_color;
}"#,
                                           None)?;

        Ok(RenderTarget {
            display: display,
            program: program,
            texture: texture,
        })
    }

    // TODO: Cache this
    fn get_rectangle_shape(&mut self,
                           _component: &Component,
                           size: &Rect)
                           -> Result<(IndexBuffer<u8>, VertexBuffer<Vertex2D>)> {
        // self.left += 1f32;
        let (left, top, right, bottom) = size.calculate_screen_positions();

        // TODO: This indexbuffer is probably always the same,
        // it might be faster to cache it / put it in the shader
        Ok((IndexBuffer::new(&self.display, PrimitiveType::TriangleStrip, &[0, 1, 2, 3])?,
            VertexBuffer::new(&self.display,
                              &[Vertex2D {
                                    position: [left, top],
                                    tex_coords: [0.0, 0.0],
                                },
                                Vertex2D {
                                    position: [right, top],
                                    tex_coords: [1.0, 0.0],
                                },
                                Vertex2D {
                                    position: [left, bottom],
                                    tex_coords: [0.0, 1.0],
                                },
                                Vertex2D {
                                    position: [right, bottom],
                                    tex_coords: [1.0, 1.0],
                                }])?))
    }

    fn draw_component(&mut self,
                      target: &mut Render,
                      frame: &mut Frame,
                      component: &Component,
                      parent_size: &Rect)
                      -> Result<()> {
        let own_size = Rect::calculate_from_parent(parent_size, component);
        let coordinates = self.get_rectangle_shape(component, &own_size)?;

        frame.draw(&coordinates.1,
                  &coordinates.0,
                  &self.program,
                  &uniform! { component_color: component.color.to_vector() },
                  &DrawParameters::default())?;

        for child in &component.children {
            let child = try_get!(target.get_component_by_id(*child),
                                 could_not_find_component(*child));
            self.draw_component(target, frame, &child, &own_size)?;
        }

        Ok(())
    }

    pub fn render(&mut self, target: &mut Render) -> Result<()> {
        let mut frame = self.display.draw();
        let component = match target.get_root_component_id() {
            Some(id) => id,
            None => {
                println!("Could not get root component ID");
                return Ok(());
            }
        };
        let component = match target.get_component_by_id(component) {
            Some(c) => c,
            None => {
                println!("Could not get root component (got ID)");
                return Ok(());
            }
        };

        let (screen_width, screen_height) =
            try_get!(try_get!(self.display.get_window(), no_window()).get_inner_size_pixels(),
                     no_inner_pixel_size());
        let (screen_width, screen_height) = (screen_width as SizeType, screen_height as SizeType);
        let screen_size = Rect {
            x: 0,
            y: 0,
            width: screen_width,
            height: screen_height,
            screen_width: screen_width,
            screen_height: screen_height,
        };

        self.draw_component(target, &mut frame, &component, &screen_size)?;

        frame.finish()?;
        Ok(())
    }
}
