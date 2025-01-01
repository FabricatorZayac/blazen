use crate::gfx::Render;
use crate::message::{InputHandler, MessageHandler};

mod menu;
mod demo;
// mod deck_scene;

pub use menu::*;
pub use demo::*;
// pub use deck_scene::*;

pub trait Scene : Render + MessageHandler + InputHandler {
    fn update(&mut self);
}

pub trait ScenePtr {
    fn init(self);
    fn get(self) -> &'static mut dyn Scene;
}
