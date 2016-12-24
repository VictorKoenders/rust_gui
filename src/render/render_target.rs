use glium::{ DisplayBuild, IndexBuffer, Frame, VertexBuffer, Program, Surface };
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use super::{ Render, Vertex2D };
use glium::glutin::WindowBuilder;
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use components::Component;


pub struct RenderTarget {
  pub display: GlutinFacade,
  pub program: Program,
  pub texture: Texture2d,
}

impl RenderTarget {
  pub fn new() -> RenderTarget {
    let display = WindowBuilder::new()
      .with_vsync()
      .build_glium()
      .unwrap();

    // TODO: Get rid of texture and just pass a color to the shader
    let program = Program::from_source(
      &display,
      r#"#version 140
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
//uniform mat4 matrix;

void main() {
	v_tex_coords = tex_coords;
	gl_Position = /*matrix * */vec4(position, 0.0, 1.0);
}
"#,
      r#"#version 140
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;

void main() {
	color = texture(tex, v_tex_coords);
}"#,
      None
    ).unwrap();
    let texture = Texture2d::new(&display, vec![vec![255u8]]).unwrap();

    RenderTarget {
      display: display,
      program: program,
      texture: texture,
    }
  }

  // TODO: Cache this
  fn get_rectangle_shape(&mut self, component: &Component) -> (IndexBuffer<u8>, VertexBuffer<Vertex2D>) {
    //self.left += 1f32;
    let (screen_width, screen_height) = self.display.get_window().unwrap().get_inner_size_pixels().unwrap();
    let (screen_width, screen_height) = (screen_width as f32, screen_height as f32);

    let left = {
      let left = component.left.get_pixel_position(0f32, screen_width) as f32;
      (left / screen_width ) * 2f32 - 1f32
    };

    let top = {
      let top = component.top.get_pixel_position(0f32, screen_height) as f32;
      1f32 - (top / screen_height) * 2f32
    };

    let right = {
      let right = if component.right.is_known() {
        left + (screen_width - left) - component.right.get_pixel_position(0f32, screen_width - left) as f32
      } else {
        component.width.get_pixel_position(0f32, screen_width) as f32
      };
      (right / screen_width ) * 2f32 - 1f32
    };

    let bottom = {
      let bottom = if component.bottom.is_known() {
        top + (screen_height - top) - component.bottom.get_pixel_position(0f32, screen_height - top) as f32
      } else {
        component.height.get_pixel_position(0f32, screen_height) as f32
      };
      1f32 - (bottom / screen_height) * 2f32
    };

    (
      IndexBuffer::new(&self.display, PrimitiveType::TriangleStrip, &[
        0, 1, 2, 3,
      ]).unwrap(),
      VertexBuffer::new(&self.display, &[
        Vertex2D { position: [left, top], tex_coords: [0.0, 0.0] },
        Vertex2D { position: [right, top], tex_coords: [1.0, 0.0] },
        Vertex2D { position: [left, bottom], tex_coords: [0.0, 1.0] },
        Vertex2D { position: [right, bottom], tex_coords: [1.0, 1.0] },
      ]).unwrap()
    )
  }

  fn draw_component(&mut self, _target: &mut Render, frame: &mut Frame, component: &Component){
    let coordinates = self.get_rectangle_shape(component);
    frame.draw(
      &coordinates.1,
      &coordinates.0,
      &self.program,
      &uniform! {}, // TODO: Deal with unwrap
      &DrawParameters::default()
    ).unwrap();
  }

  pub fn render(&mut self, target: &mut Render) {
    let mut frame = self.display.draw();
    let component = match target.get_root_component_id() {
      Some(id) => id,
      None => {
        println!("Could not get root component ID");
        return;
      }
    };
    let component = match target.get_component_by_id(component) {
      Some(c) => c,
      None => {
        println!("Could not get root component (got ID)");
        return;
      }
    };

    self.draw_component(target, &mut frame, &component);

    frame.finish().unwrap();
  }
}