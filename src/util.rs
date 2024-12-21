#[derive(Debug)]
pub struct Angle(f64);

impl Angle {
    pub fn from_rad(rad: f64) -> Self {
        Self(rad)
    }
    pub fn from_deg(deg: f64) -> Self {
        Self(core::f64::consts::PI / 180.0 * deg)
    }
    pub fn as_rad(&self) -> f64 {
        self.0
    }
    pub fn as_deg(&self) -> f64 {
        self.0 * 180.0 / core::f64::consts::PI
    }
}

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

    pub fn step(&mut self) {
        self.0 += 1;
    }
}
