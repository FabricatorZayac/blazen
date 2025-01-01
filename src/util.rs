use core::cell::Cell;

use wasm4::control::{Mouse, MouseState};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Duration(u32);

impl Duration {
    pub fn from_secs(secs: f32) -> Self {
        Self((secs * 60.0) as u32)
    }
    pub fn from_frames(frames: u32) -> Self {
        Self(frames)
    }

    pub fn as_frames(&self) -> u32 {
        self.0
    }
    pub fn as_secs(&self) -> f32 {
        self.0 as f32 / 60.0
    }
}

static mut FRAME_COUNT: u32 = 0;
pub struct FrameCounter;
impl FrameCounter {
    pub fn get() -> u32 {
        unsafe { FRAME_COUNT }
    }
    pub(super) fn increment() {
        unsafe { FRAME_COUNT += 1 };
    }
}

static mut ENTROPY: [u8; 16] = [0; 16];
pub struct Entropy;
impl Entropy {
    pub fn get() -> [u8; 16] {
        unsafe { ENTROPY }
    }
    pub(super) fn update(m: &wasm4::control::MouseState) {
        unsafe {
            ENTROPY[(FrameCounter::get() % 16) as usize] = (m.x * 10 + m.y) as u8;
        }
    }
}

pub struct MouseCompound<'a> {
    mouse: &'a Mouse,
    prev: MouseState,
}
impl<'a> MouseCompound<'a> {
    pub(super) fn new(mouse: &'a Mouse, prev: MouseState) -> Self {
        MouseCompound { mouse, prev }
    }

    pub fn state(&self) -> MouseState {
        self.mouse.state()
    }
    pub fn prev(&self) -> MouseState {
        self.prev
    }
}
