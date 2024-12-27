use crate::{gfx::Render, MouseSemaphore};

pub mod menu;
pub mod demo;

pub use menu::*;
pub use demo::*;

pub trait Scene : Render {
    fn update(&mut self, m: &MouseSemaphore);
}

