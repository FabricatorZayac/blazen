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
