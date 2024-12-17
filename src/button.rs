use wasm4::{draw::{DrawIndex, Framebuffer}, rt::Runtime, sys, trace};

use crate::{gfx::Render, Blazen};

pub struct Button<F: FnMut(&mut Blazen)> {
    start: [i32; 2],
    text: &'static str,
    fill: DrawIndex,
    outline: DrawIndex,
    text_color: DrawIndex,
    onclick: F,
}

impl<F: FnMut(&mut Blazen)> Render for Button<F> {
    fn render(self, fb: &Framebuffer) {
        fb.rect(
            self.start,
            self.get_shape(),
            self.fill,
            self.outline,
        );

        fb.text(
            self.text,
            [self.start[0] + 2, self.start[1] + 2],
            self.text_color,
            self.fill,
        );
    }
}

impl<F: FnMut(&mut Blazen)> Button<F> {
    pub fn new(
        start: [i32; 2],
        text: &'static str,
        fill: DrawIndex,
        outline: DrawIndex,
        text_color: DrawIndex,
        onclick: F,
    ) -> Self {
        Self {
            start,
            text,
            fill,
            outline,
            text_color,
            onclick,
        }
    }
    fn get_shape(&self) -> [u32; 2] {
        [(self.text.len() * 8 + 3) as u32, 11]
    }
    pub fn update(mut self, rt: &mut Blazen) -> Self {
        let mouse = rt.mouse.state();

        if !mouse.buttons.left {
            return self;
        }
        if let Some(state) = &rt.prev_mouse {
            if state.buttons.left {
                return self;
            }
        }

        let left = mouse.x as i32 > self.start[0];
        let right = (mouse.x as u32) < self.get_shape()[0] + self.start[0] as u32;
        let bottom = mouse.y as i32 > self.start[1];
        let top = (mouse.y as u32) < self.get_shape()[1] + self.start[1] as u32;
        if left && right {
            if top && bottom {
                (self.onclick)(rt);
            }
        }

        self
    }
}
