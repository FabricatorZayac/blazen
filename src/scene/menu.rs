use core::mem::MaybeUninit;

use crate::{button::Button, card::{card::{Card, Rank, Suit}, state::{idle1, CardState}}, gfx::Render, message::Message, MouseSemaphore};

use super::Scene;

static mut MENU: MaybeUninit<Menu> = MaybeUninit::uninit();

pub struct Menu {
    ace: CardState,
    start: Button,
}
impl Menu {
    pub fn init() {
        unsafe {
            MENU = MaybeUninit::new(Self::new());
        }
    }
    fn new() -> Self {
        Self {
            ace: CardState::new(
                Card::new(Suit::Spade, Rank::Ace),
                [80, 60],
                Some(idle1()),
            ),
            start: Button::new(
                [30, 100],
                "Start",
                wasm4::draw::DrawIndex::Third,
                wasm4::draw::DrawIndex::Second,
                Message::Start,
            ),
        }
    }
    pub fn get() -> &'static mut Self {
        unsafe {
            MENU.assume_init_mut()
        }
    }
}
impl Scene for Menu {
    fn update(&mut self, m: &MouseSemaphore) {
        // FIXME: I don't know why this crashes
        // self.ace.handle_input(m);
        self.ace.update();

        self.start.update(m);
    }
}
impl Render for Menu {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        self.start.render(fb);
        self.ace.render(fb);
    }
}
