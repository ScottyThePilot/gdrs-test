extern crate gdnative;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloRust;

#[methods]
impl HelloRust {
  fn new(_base: &Node) -> Self {
    HelloRust
  }

  #[method]
  fn _ready(&self, #[base] _base: &Node) {
    godot_print!("Hello from Rust");
  }
}

fn init(handle: InitHandle) {
  handle.add_class::<HelloRust>();
}

godot_init!(init);
