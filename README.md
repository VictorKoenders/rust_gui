# Rust GUI
A framework to write GUI applications in idiomatic rust.

```
#[macro_use] extern crate gui;

use gui::render::Render;

pub fn main(){
  let mut view = layout!(Component {
    left: 50 px,
    top: 50 px,
    right: 50 px,
    bottom: 50 px,
    color: red,

    Component {
      left: 50 px,
      right: 50 px,
      top: 50 px,
      bottom: 50 px,
      color: green,

      Component {
        left: 50 px,
        right: 50 px,
        top: 50 px,
        bottom: 50 px,
        color: blue,
      }
    }
  });

  let mut render = Render::new();
  render.mount(&mut view);

  render.render().unwrap();
}
```

![screenshot](https://raw.githubusercontent.com/VictorKoenders/rust_gui/master/examples/basic/screenshot.png)