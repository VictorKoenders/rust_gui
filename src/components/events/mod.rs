mod click;
mod load;

use std::collections::HashMap;
pub use self::click::*;
pub use self::load::*;

#[derive(Default)]
pub struct EventMap {
    pub click: HashMap<u64, Vec<TClickEvent>>,
    pub load: HashMap<u64, Vec<TLoadEvent>>,
}

impl EventMap {
    pub fn new() -> EventMap {
        EventMap {
            click: HashMap::new(),
            load: HashMap::new(),
        }
    }
}
