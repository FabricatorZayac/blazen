use crate::message::{Message, MessageHandler, Reader, Writer};
use crate::gfx::Render;
use crate::card::{deck::Deck, state::CardState, Suit};

use super::{Scene, DEMO};

pub const DECK_VIEW: *mut DeckScene = DEMO.wrapping_add(1) as *mut DeckScene;
pub struct DeckScene {
    page: heapless::Vec<CardState, 64>,
}
impl DeckScene {
    pub fn get() -> &'static mut Self {
        unsafe {
            DECK_VIEW.as_mut().unwrap()
        }
    }
}

impl Scene for DeckScene {
    fn update(&mut self, mouse: &crate::MouseSemaphore, tx: &mut Writer, _rx: &Reader) {
        if let Some(m) = mouse.state() {
            if m.buttons.left && !mouse.prev().buttons.left {
                tx.write(Message::BackToGame).ok();
            }
        }
    }
}

impl MessageHandler for DeckScene {
    fn handle_message(&mut self, _rx: &Reader) { }
}

impl Render for DeckScene {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        self.page.iter().for_each(|spade| spade.render(fb));
    }
}

impl From<&Deck> for &'static mut dyn Scene {
    fn from(value: &Deck) -> Self {
        let this = DeckScene::get();
        let spade_n = value.iter()
            .filter(move |card| card.suit() == Suit::Spade)
            .count();

        for (i, spade) in value
            .iter()
            .filter(move |card| card.suit() == Suit::Spade)
            .enumerate() {
            let x = 70 + (i * 160 / spade_n - 50);
            this.page.push(CardState::new(i, spade.clone(), [x as i32, 20], None)).ok();
        }

        this
    }
}
