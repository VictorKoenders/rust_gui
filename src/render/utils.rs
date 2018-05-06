use components::{SizeType};

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: SizeType,
    pub y: SizeType,
    pub width: SizeType,
    pub height: SizeType,

    pub screen_width: SizeType,
    pub screen_height: SizeType,
}

impl Rect {
    pub fn calculate_screen_positions(&self) -> (f32, f32, f32, f32) {
        (((self.left() as f32) / (self.screen_width as f32) * 2f32) - 1f32,
         ((self.top() as f32) / (self.screen_height as f32) * 2f32) - 1f32,
         ((self.right() as f32) / (self.screen_width as f32) * 2f32) - 1f32,
         ((self.bottom() as f32) / (self.screen_height as f32) * 2f32) - 1f32)
    }

    pub fn left(&self) -> SizeType {
        self.x
    }
    pub fn right(&self) -> SizeType {
        self.x + self.width
    }
    pub fn top(&self) -> SizeType {
        self.y
    }
    pub fn bottom(&self) -> SizeType {
        self.y + self.height
    }
/*
    pub fn calculate_from_parent(parent: &Rect, component: &Component) -> Rect {
        let left = {
            if component.left.is_known() {
                component.left.get_pixel_position(parent.left(), parent.right())
            } else if component.right.is_known() && component.width.is_known() {
                let right = component.right
                    .get_pixel_position_reversed(parent.left(), parent.right());
                let width = component.width.get_pixel_position(parent.left(), right);
                right - width
            } else {
                parent.left()
            }
        };

        let top = {
            if component.top.is_known() {
                component.top.get_pixel_position(parent.top(), parent.bottom())
            } else if component.bottom.is_known() && component.height.is_known() {
                let bottom = component.bottom
                    .get_pixel_position_reversed(parent.top(), parent.bottom());
                let height = component.width.get_pixel_position(parent.top(), bottom);
                bottom - height
            } else {
                parent.top()
            }
        };

        let width = {
            if component.width.is_known() {
                component.width.get_pixel_position(left, parent.right())
            } else if component.right.is_known() {
                let right = component.right.get_pixel_position_reversed(left, parent.right());
                right - left
            } else {
                parent.right() - left
            }
        };

        let height = {
            if component.height.is_known() {
                component.height.get_pixel_position(top, parent.bottom())
            } else if component.bottom.is_known() {
                let bottom = component.bottom.get_pixel_position_reversed(top, parent.bottom());
                bottom - top
            } else {
                parent.bottom() - top
            }
        };

        Rect {
            x: left,
            y: top,
            width: width,
            height: height,

            screen_width: parent.screen_width,
            screen_height: parent.screen_height,
        }
    }*/
}
