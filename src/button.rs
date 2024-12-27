use wasm4::draw::{DrawIndex, Framebuffer};

use crate::{gfx::Render, message::{Message, MESSAGE_BUF}, MouseSemaphore};

#[derive(derive_new::new)]
pub struct Button {
    start: [i32; 2],
    text: &'static str,
    fill: DrawIndex,
    outline: DrawIndex,
    onclick: Message,
}

impl Render for Button {
    fn render(&self, fb: &Framebuffer) {
        fb.rect(self.start, self.get_shape(), self.fill, self.outline);

        fb.text(
            self.text,
            [self.start[0] + 2, self.start[1] + 2],
            DrawIndex::Fourth,
            self.fill,
        );
    }
}
impl Button {
    pub fn update(&mut self, mouse: &MouseSemaphore) {
        let m = mouse.state().unwrap();

        if !m.buttons.left {
            return;
        }
        if let Some(state) = &mouse.prev {
            if state.buttons.left {
                return;
            }
        }

        let left = m.x as i32 > self.start[0];
        let right = (m.x as u32) < self.get_shape()[0] + self.start[0] as u32;
        let bottom = m.y as i32 > self.start[1];
        let top = (m.y as u32) < self.get_shape()[1] + self.start[1] as u32;
        if left && right && top && bottom {
            unsafe { MESSAGE_BUF = Some(self.onclick) }
        }
    }
    fn get_shape(&self) -> [u32; 2] {
        [(self.text.len() * 8 + 3) as u32, 11]
    }
}
