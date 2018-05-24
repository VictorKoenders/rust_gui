pub struct ComponentState {
    dirty: bool,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl ComponentState {
    get_set!(
        #[doc = "Get the x position of this component"]
        x,
        #[doc = "Set the x position of this component"]
        set_x,
        f32,
        x
    );
    get_set!(
        #[doc = "Get the left edge of this component.\n\nThis is equal to [ComponentState::x]"]
        left,
        #[doc = "Set the left edge of this component.\n\nThis is equal to [ComponentState::set_x]"]
        set_left,
        f32,
        x
    );
    get_set!(
        #[doc = "Get the y position of this component"]
        y,
        #[doc = "Set the y position of this component"]
        set_y,
        f32,
        y
    );
    get_set!(
        #[doc = "Get the top edge of this component.\n\nThis is equal to [ComponentState::y]"]
        top,
        #[doc = "Set the y position of this component.\n\nThis is equal to [ComponentState::set_y]"]
        set_top,
        f32,
        y
    );
    get_set!(
        #[doc = "Get the width of this component"]
        width,
        #[doc = "Set the width of this component"]
        set_width,
        f32,
        width
    );
    get_set!(
        #[doc = "Get the height of this component"]
        height,
        #[doc = "Set the height of this component"]
        set_height,
        f32,
        height
    );

    /// Checks whether this component is "dirty", e.g. if it is modified and requires updating
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the value of 'right' on this component
    ///
    /// This is equal to `self.x + self.width`
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Set the value of 'right' on this component
    ///
    /// Note: this sets the width in regards to the x position. If you want to change the x position, call [ComponentState::x] or [ComponentState::set_left] instead
    pub fn set_right(&mut self, value: f32) {
        self.dirty = true;
        self.width = value - self.x;
    }

    /// Get the value of 'bottom' on this component
    ///
    /// This is equal to `self.y + self.height`
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
    /// Set the value of 'bottom' on this component
    ///
    /// Note: this sets the height in regards to the y position. If you want to change the y position, call [ComponentState::set_y] or [ComponentState::set_top] instead
    pub fn set_bottom(&mut self, value: f32) {
        self.dirty = true;
        self.height = value - self.y;
    }
}
