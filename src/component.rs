use states::{Click, Render};
use error::Result;

pub trait Component {
    fn render(&self, render: &Render) -> Result<()>;
    fn click(&mut self, click: &Click) -> Result<()>;
}
