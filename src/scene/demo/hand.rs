use derive_new::new;

use crate::{animator::{animation_state::AnimationState, transform::Translate}, card::{card::Card, state::{idle1, CardState}}, gfx::Render, util::Duration};

pub struct Hand {
    pub size: usize,
    pub cards: heapless::Vec<Card, 10>,
}

#[derive(new)]
pub struct HandState {
    pub size: usize,
    pub cards: heapless::Vec<CardState, 10>,
}
impl HandState {
    pub fn push(&mut self, card: Card) {
        let pos = self.cards.len();
        let origin = [20 + pos as i32 * 15, 130];
        self.cards.push(CardState::new(
            card,
            origin,
            Some(AnimationState::new(&[Translate::new(
                [80.0 - origin[0] as f64, 0.0 - origin[1] as f64],
                [0.0, 0.0],
            ).into()], Duration::from_secs(0.1), Some(idle1))),
        )).ok();
    }
}

impl From<Hand> for HandState {
    fn from(mut value: Hand) -> Self {
        value.cards.sort_unstable();

        Self {
            size: value.size,
            cards: value.cards
                .into_iter()
                .enumerate()
                .map(|(i, card)| {
                    let origin = [20 + i as i32 * 15, 130];
                    CardState::new(card, origin,
                    Some(AnimationState::new(&[Translate::new(
                        [80.0 - origin[0] as f64, 0.0 - origin[1] as f64],
                        [0.0, 0.0],
                    ).into()], Duration::from_secs(0.1), Some(idle1))))
                }).collect(),
        }
    }
}

impl Render for HandState {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        self.cards.iter().for_each(|card| card.render(fb));
    }
}

// NOTE: Batching like this doesn't work because it
// overwrites TEXTURE_BUFFER for each card
// impl HandState {
//     pub fn view(&self) -> HandView {
//         HandView(self.cards.iter().map(CardState::view).collect())
//     }
// }
// pub struct HandView(heapless::Vec<CardView, 10>);
// impl Render for HandView {
//     fn render(&self, fb: &wasm4::draw::Framebuffer) {
//         self.0.iter().for_each(|view| view.render(fb));
//     }
// }
