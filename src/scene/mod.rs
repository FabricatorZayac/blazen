use crate::{gfx::Render, MouseSemaphore};

mod menu;
mod demo;
mod deck_scene;

pub use menu::*;
pub use demo::*;
pub use deck_scene::*;

pub trait Scene : Render {
    fn update(&mut self, m: &MouseSemaphore);
}

pub trait ScenePtr {
    fn init(self);
}
