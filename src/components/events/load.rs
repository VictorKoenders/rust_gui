use super::super::View;

pub type TLoadEvent = Box<Fn(&mut View, u64, &mut LoadEvent)>;

#[derive(Debug, Clone)]
pub struct LoadEvent {
}
