
macro_rules! get_set {
    (#[$get_meta:meta] $get_name:ident, #[$set_meta:meta] $set_name:ident, $type:ty, $field:ident) => {
        #[$get_meta]
        pub fn $get_name(&self) -> $type {
            self.$field
        }
        #[$set_meta]
        pub fn $set_name(&mut self, value: $type) {
            self.dirty = true;
            self.$field = value;
        }
    };

    (ref $get_name:ident, $set_name:ident, $type:ty, $field:ident) => {
        /// Get a reference to the value of $get_name of this component
        pub fn $get_name(&self) -> &$type {
            &self.$field
        }
        /// Set the value of $get_name of this component
        pub fn $set_name(&mut self, value: $type) {
            self.dirty = true;
            self.$field = value;
        }
    }
}
