use core::mem::MaybeUninit;

use crate::{
    button::Button,
    card::{animations::idle1, state::{CardData, CardState}, Card, Rank, Suit},
    gfx::Render,
    message::{InputHandler, Message, MessageHandler, Reader, Writer},
    MouseCompound,
};

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
                0,
                CardData::Playing(Card::new(Suit::Spade, Rank::Ace)),
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
impl MessageHandler for Menu {
    fn handle_message(&mut self, rx: &Reader) {
        self.ace.handle_message(rx);
    }
}
impl InputHandler for Menu {
    fn handle_input(&self, mouse: &MouseCompound, tx: &mut Writer) {
        self.ace.handle_input(mouse, tx);
        self.start.handle_input(mouse, tx);
    }
}
impl Scene for Menu {
    fn update(&mut self) {
        self.ace.update();
    }
}
impl Render for Menu {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        self.ace.render(fb);
        self.start.render(fb);
    }
}
