use wasm4::tracef;

use crate::{card::{joker::{Joker, JokerType}, state::{CardData, CardState}}, gfx::Render, message::{InputHandler, MessageHandler}};

pub struct Jokers {
    size: usize,
    jokers: heapless::Vec<CardState, 10>,
}

impl Jokers {
    pub fn update(&mut self) {
        self.jokers
            .iter_mut()
            .for_each(CardState::update);
    }
}

impl Default for Jokers {
    fn default() -> Self {
        let mut jokers = heapless::Vec::new(); 

        // For testing purposes
        jokers.push(CardState::new(
            // Jokers ids starting at 0x100
            0x100,
            CardData::Joker(Joker::new(JokerType::Jimbo)),
            [80, 44],
            None,
        )).unwrap();

        Self { size: 5, jokers }
    }
}

impl InputHandler for Jokers {
    fn handle_input(&self, mouse: &crate::util::MouseCompound, tx: &mut crate::message::Writer) {
        self.jokers
            .iter()
            .for_each(|joker| joker.handle_input(mouse, tx));
    }
}

impl MessageHandler for Jokers {
    fn handle_message(&mut self, rx: &crate::message::Reader) {
        self.jokers
            .iter_mut()
            .for_each(|joker| joker.handle_message(rx));
    }
}

impl Render for Jokers {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        self.jokers
            .iter()
            .for_each(|joker| joker.render(fb));
    }
}
