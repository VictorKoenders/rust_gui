use canvas_components::{CanvasPath, FillStyle, StrokeStyle};
use error::Result;
use point::Point;

pub struct Canvas;

impl Canvas {
    /// Get the height of the canvas, in pixels
    pub fn height(&self) -> u32 {
        unimplemented!()
    }

    /// Get the width of the canvas, in pixels
    pub fn width(&self) -> u32 {
        unimplemented!()
    }

    /// Draw an arc at position x/y with a radius.
    /// The x/y position is the center point of the circle that the arc is a part of.
    /// 
    /// startAngle and endAngle determine which part of the arc will be rendered.
    /// 
    /// This arc is always rendered clockwise, so if startAngle < endAngle, the arc will wrap around. For a counter-clockwise implementation, see [Canvas::arc_cc].
    pub fn arc(&mut self, x: u32, y: u32, radius: f32, start_angle: f32, end_angle: f32) -> Result<()> {
        unimplemented!()
    }
 
    /// Draw a counter-clockwise arc at position x/y with a radius.
    /// The x/y position is the center point of the circle that the arc is a part of.
    /// 
    /// startAngle and endAngle determine which part of the arc will be rendered.
    /// 
    /// This arc is always rendered counter clockwise, so if startAngle > endAngle, the arc will wrap around. For a clockwise implementation, see [Canvas::arc].
    pub fn arc_cc(&mut self, x: u32, y: u32, radius: f32, start_angle: f32, end_angle: f32) -> Result<()> {
        unimplemented!()
    }

    /// Draw a filled rectangle with a given fillstyle
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, style: FillStyle) -> Result<()> {
        unimplemented!()
    }

    /// Draw a rectangle with a given strokestyle
    pub fn stroke_rect(&mut self, x: u32, y: u32, width: u32, height: u32, style: StrokeStyle) -> Result<()> {
        unimplemented!()
    }

    /// Render a text with the given fill style
    pub fn fill_text(&mut self, point: impl Into<Point>, text: impl AsRef<str>, style: FillStyle) -> Result<()> {
        unimplemented!()
    }

    /// Render a text with the given stroke style
    pub fn stroke_text(&mut self, point: impl Into<Point>, text: impl AsRef<str>, style: StrokeStyle) -> Result<()> {
        unimplemented!()
    }

    /// Begin a path renderer.
    pub fn begin_path(&mut self) -> CanvasPath {
        unimplemented!()
    }
}
