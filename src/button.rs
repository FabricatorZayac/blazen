use wasm4::draw::{DrawIndex, Framebuffer};

pub struct Button<F> {
    start: [i32; 2],
    text: &'static str,
    fill: DrawIndex,
    outline: DrawIndex,
    text_color: DrawIndex,
    on_click: F,
}

impl<F: FnMut()> Button<F> {
    pub fn new(
        start: [i32; 2],
        text: &'static str,
        fill: DrawIndex,
        outline: DrawIndex,
        text_color: DrawIndex,
        on_click: F,
    ) -> Self {
        Self {
            start,
            text,
            fill,
            outline,
            text_color,
            on_click,
        }
    }
    pub fn draw(&self, framebuffer: &Framebuffer) {
        let shape: [u32; 2] = [(self.text.len() * 8 + 3) as u32, 11];
        framebuffer.rect(
            self.start,
            shape,
            self.fill,
            self.outline,
        );

        framebuffer.text(
            self.text,
            [self.start[0] + 2, self.start[1] + 2],
            self.text_color,
            self.fill,
        );
    }
}
