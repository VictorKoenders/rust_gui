use super::{Component, EventMap};
use std::collections::HashMap;
use render::Render;

#[derive(Default)]
pub struct Container {
    pub view: View,
    pub events: EventMap,
}

impl Container {
    pub fn new() -> Container {
        Container {
            view: View::new(),
            events: EventMap::new(),
        }
    }

    pub fn mount(&mut self, _render_target: &mut Render) {
        for event in &self.events.load {
            let component_id = match self.view.get_component(*event.0) {
                Some(c) => c.id,
                None => continue,
            };
            for callback in event.1 {
                callback(&mut self.view, component_id, &mut super::LoadEvent {});
            }
        }
    }

    pub fn trigger_click_on(&mut self, id: u64) {
        let click_event = match self.events.click.get(&id) {
            Some(c) => c,
            None => return,
        };

        let mut event = super::ClickEvent {
            button: super::MouseButton::Left,
            client_x: 0,
            client_y: 0,
            screen_x: 0,
            screen_y: 0,
            consumed: false,
        };

        for click_event in click_event {
            click_event(&mut self.view, id, &mut event);
            if event.consumed {
                break;
            }
        }
    }
}

impl ::std::fmt::Debug for Container {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "{:?}", self.view)
    }
}

#[derive(Debug, Default)]
pub struct View {
    root_component: Option<Component>,
    components: HashMap<u64, Component>,
}

impl View {
    pub fn new() -> View {
        View {
            root_component: None,
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.insert(component.id, component);
    }

    // TODO: If we're rendering, notify that this component could have been updated
    pub fn get_parent(&self, child_component: &Component) -> Option<&Component> {
        if child_component.parent == super::component::COMPONENT_NONE {
            None
        } else {
            self.components.get(&child_component.parent)
        }
    }

    // TODO: If we're rendering, notify that this component could have been updated
    pub fn get_parent_mut(&mut self, child_component: &Component) -> Option<&mut Component> {
        if child_component.parent == super::component::COMPONENT_NONE {
            None
        } else {
            self.components.get_mut(&child_component.parent)
        }
    }

    // TODO: If we're rendering, notify that this component could have been updated
    pub fn get_component(&self, id: u64) -> Option<&Component> {
        self.components.get(&id)
    }

    // TODO: If we're rendering, notify that this component could have been updated
    pub fn get_component_mut(&mut self, id: u64) -> Option<&mut Component> {
        self.components.get_mut(&id)
    }

    pub fn get_root_id(&self) -> Option<u64> {
        match self.root_component {
            Some(ref c) => Some(c.id),
            None => None,
        }
    }

    pub fn set_root_component(&mut self, component: Component) {
        self.root_component = Some(component);
    }
}
