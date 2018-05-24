pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl From<(i32, i32)> for Point {
    fn from(val: (i32, i32)) -> Point {
        Point {
            x: val.0 as f32,
            y: val.1 as f32,
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from(val: (f32, f32)) -> Point {
        Point {
            x: val.0,
            y: val.1,
        }
    }
}

