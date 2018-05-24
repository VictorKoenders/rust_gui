use canvas::Canvas;
use error::Result;
use point::Point;
use super::{FillStyle, StrokeStyle};

pub struct CanvasPath<'a> {
    canvas: &'a mut Canvas,
}

impl<'a> CanvasPath<'a> {
     /// Move to the giving position, without drawing a line
    pub fn move_to(&mut self, point: impl Into<Point>) -> &mut CanvasPath {
        unimplemented!()
    }

    /// Draw a line from the current position to the given position
    pub fn line_to(&mut self, point: impl Into<Point>) -> &mut CanvasPath {
        unimplemented!()
    }

    /// Draw a cubic bezier curve to the path.
    pub fn bezier_curve_to(&mut self, control_point_1: impl Into<Point>, control_point_2: impl Into<Point>, end: impl Into<Point>) -> &mut CanvasPath {
        unimplemented!()
    }

    pub fn arc(&mut self, start: impl Into<Point>, end: impl Into<Point>, radius: f32) -> &mut CanvasPath {
        unimplemented!()
    }

    /// Close the path, drawing a line from the current point to the starting point
    pub fn close(&mut self) -> &mut CanvasPath {
        unimplemented!()
    }

    pub fn contains_point(&self, point: impl Into<Point>) -> bool {
        unimplemented!()
    }
    
    /// strokes the current path with the current stroke style
    pub fn stroke(mut self, style: StrokeStyle) -> Result<()> {
        unimplemented!()
    }

    /// Fills the current area with the current fill style
    pub fn fill(mut self, style: FillStyle) -> Result<()> {
        unimplemented!()
    }
}