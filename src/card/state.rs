use constgebra::CMatrix;
use wasm4::control::MouseState;

use crate::animator::transform::Scale;
use crate::gfx::texture::Texture;
use crate::message::{InputHandler, Message, MessageHandler, Writer};
use crate::util::Duration;
use crate::util::MouseCompound;
use crate::animator::animation_state::AnimationState;
use crate::gfx::{Render, Triangle, TriangleFill, Vectorize};

use super::animations::idle1;
use super::card::Card;
use super::joker::Joker;

#[derive(Debug)]
pub enum CardData {
    Playing(Card),
    Joker(Joker),
}
impl CardData {
    pub fn texture(&self) -> [Texture; 2] {
        match self {
            CardData::Playing(card) => card.texture(),
            CardData::Joker(joker) => joker.texture(),
        }
    }
}

#[derive(Debug)]
pub struct CardState {
    id: usize,
    card: CardData,
    origin: [i32; 2],
    diff_vecs: [[f64; 2]; 4],

    animation: Option<AnimationState>,
}

impl CardState {
    pub fn new(id: usize, card: CardData, origin: [i32; 2], animation: Option<AnimationState>) -> Self {
        Self {
            id,
            card,
            origin,
            diff_vecs: [
                [-16.0, -21.0],
                [ 16.0, -21.0],
                [ 16.0,  21.0],
                [-16.0,  21.0],
            ],
            animation,
        }
    }
    fn animate(&self) -> CMatrix<3, 3> {
        self.animation
            .as_ref()
            .map_or(Some(CMatrix::identity()), AnimationState::update)
            .unwrap_or(CMatrix::identity())
    }
    fn apply_transform(&self, matrix: CMatrix<3, 3>) -> [[f64; 2]; 4] {
        self.diff_vecs
            .map(Vectorize::vectorize)
            .map(|vec| vec.mul(matrix))
            .map(Vectorize::devectorize)
    }
    pub fn is_hovered(&self, mouse: MouseState) -> bool {
        let left   = (mouse.x as i32) > self.origin[0] - 15;
        let right  = (mouse.x as i32) < self.origin[0] + 15;
        let bottom = (mouse.y as i32) > self.origin[1] - 20;
        let top    = (mouse.y as i32) < self.origin[1] + 20;
        left && right && bottom && top
    }

    pub fn origin(&self) -> [i32; 2] {
        self.origin
    }

    pub fn set_origin(&mut self, origin: [i32; 2]) {
        self.origin = origin;
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl CardState {
    pub fn set_animation(&mut self, animation: AnimationState) {
        self.animation = Some(animation);
    }
    pub fn update(&mut self) {
        // default card animation
        if self.animation.is_none() {
            self.set_animation(idle1());
        }

        // replace animation when it ends
        let mut next: Option<AnimationState> = None;
        if let Some(anim) = &self.animation {
            if anim.finished() {
                next = anim.get_next()
            }
        }
        if let Some(anim) = next {
            self.set_animation(anim);
        }
    }
}

impl InputHandler for CardState {
    fn handle_input(&self, mouse: &MouseCompound, tx: &mut Writer) {
        let m = mouse.state();

        if self.is_hovered(m) {
            if m.buttons.left && !mouse.prev().buttons.left {
                tx.write(Message::CardClicked(self.id)).ok();
            }
            tx.write(Message::CardHovered(self.id)).ok();
        }
    }
}

impl MessageHandler for CardState {
    fn handle_message(&mut self, rx: &crate::message::Reader) {
        match rx.read() {
            Some(Message::CardHovered(id)) if id == self.id => {
                match &self.card {
                    CardData::Playing(_) => self.set_animation(AnimationState::new(
                        &[Scale::new([1.3, 1.3], [1.0, 1.0]).into()],
                        Duration::from_secs(0.1),
                        Some(idle1),
                    )),
                    CardData::Joker(_) => self.set_animation(AnimationState::new(
                        &[Scale::new([2.0, 2.0], [1.0, 1.0]).into()],
                        Duration::from_secs(0.1),
                        Some(idle1),
                    )),
                }
            }
            _ => (),
        }
    }
}

impl Render for CardState {
    fn render(&self, fb: &wasm4::draw::Framebuffer) {
        let transform = self.animate();
        let origin = self.origin.map(|i| i as f64).vectorize();
        let texture = self.card.texture();
        let vertices = self.apply_transform(transform)
            .map(Vectorize::vectorize)
            .map(|vertex| vertex.add(origin))
            .map(Vectorize::devectorize)
            .map(|vertex: [f64; 2]| vertex.map(|f| f as i32));

        let t1 = Triangle {
            vertices: [vertices[0], vertices[1], vertices[3]],
            fill: TriangleFill::Texture(texture[0]),
        };
        let t2 = Triangle {
            vertices: [vertices[1], vertices[2], vertices[3]],
            fill: TriangleFill::Texture(texture[1]),
        };

        t1.render(fb);
        t2.render(fb);
    }
}
