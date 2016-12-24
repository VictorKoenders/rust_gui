use super::super::View;

pub type TClickEvent = Box<Fn(&mut View, u64, &mut ClickEvent)>;

#[derive(Debug, Clone)]
pub struct ClickEvent {
  pub screen_x: u64,
  pub screen_y: u64,
  pub client_x: i64,
  pub client_y: i64,

  pub button: MouseButton,
  pub consumed: bool,
}

#[derive(Debug, Clone)]
pub enum MouseButton {
  Left,
  Middle,
  Right,
}