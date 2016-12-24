use glium::{ DrawError, GliumCreationError, ProgramCreationError, SwapBuffersError };
use glium::vertex::BufferCreationError as VertexCreationError;
use glium::index::BufferCreationError as IndexCreationError;
use glium::texture::TextureCreationError;
use glium::glutin::CreationError;
use std::convert::From;

#[derive(Debug)]
pub struct RenderError {
  description: String,
  error_type: RenderErrorEnum,
}

#[derive(Debug)]
pub enum RenderErrorEnum {
  GliumCreationError(GliumCreationError<CreationError>),
  ProgramCreationError(ProgramCreationError),
  TextureCreationError(TextureCreationError),
  IndexCreationError(IndexCreationError),
  VertexCreationError(VertexCreationError),
  DrawError(DrawError),
  SwapBuffersError(SwapBuffersError),
  NoWindow,
  NoInnerPixelSize,
  ComponentNotFound(u64),
}

impl RenderError {
  pub fn no_window() -> RenderError {
    RenderError {
      description: "No window found".to_owned(),
      error_type: RenderErrorEnum::NoWindow,
    }
  }
  pub fn no_inner_pixel_size() -> RenderError {
    RenderError {
      description: "No inner pixel size of WinRef found".to_owned(),
      error_type: RenderErrorEnum::NoInnerPixelSize,
    }
  }
  pub fn could_not_find_component(id: u64) -> RenderError {
    RenderError {
      description: format!("Could not find component {}", id),
      error_type: RenderErrorEnum::ComponentNotFound(id),
    }
  }
}

impl From<GliumCreationError<CreationError>> for RenderError {
  fn from(error: GliumCreationError<CreationError>) -> Self {
    RenderError {
      description: format!("Glium creation error: {:?}", error),
      error_type: RenderErrorEnum::GliumCreationError(error)
    }
  }
}

impl From<ProgramCreationError> for RenderError {
  fn from(error: ProgramCreationError) -> Self {
    RenderError {
      description: format!("Program creation error: {:?}", error),
      error_type: RenderErrorEnum::ProgramCreationError(error)
    }
  }
}

impl From<TextureCreationError> for RenderError {
  fn from(error: TextureCreationError) -> Self {
    RenderError {
      description: format!("Texture creation error: {:?}", error),
      error_type: RenderErrorEnum::TextureCreationError(error)
    }
  }
}

impl From<IndexCreationError> for RenderError {
  fn from(error: IndexCreationError) -> Self {
    RenderError {
      description: format!("Index creation error: {:?}", error),
      error_type: RenderErrorEnum::IndexCreationError(error)
    }
  }
}

impl From<VertexCreationError> for RenderError {
  fn from(error: VertexCreationError) -> Self {
    RenderError {
      description: format!("Vertex creation error: {:?}", error),
      error_type: RenderErrorEnum::VertexCreationError(error)
    }
  }
}

impl From<DrawError> for RenderError {
  fn from(error: DrawError) -> Self {
    RenderError {
      description: format!("Draw error: {:?}", error),
      error_type: RenderErrorEnum::DrawError(error)
    }
  }
}

impl From<SwapBuffersError> for RenderError {
  fn from(error: SwapBuffersError) -> Self {
    RenderError {
      description: format!("Swap buffers error: {:?}", error),
      error_type: RenderErrorEnum::SwapBuffersError(error)
    }
  }
}
