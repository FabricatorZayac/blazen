use wasm4::tracef;

pub enum Transformation {
    Rotation { start: f64, end: f64 },
}

pub trait Animate {
    fn set_rotation(&mut self, rotation: f64);
    // fn animate<F: FnMut(&mut Self)>(&mut self, mut f: F) {
    //     (f)(self);
    // }
}

pub struct Animator {
    transformation: Transformation,
    duration: u32,
    elapsed: u32,
    is_complete: bool,
}

impl Animator {
    pub fn new(transformation: Transformation, duration: u32) -> Self {
        Self {
            transformation,
            duration,
            elapsed: 0,
            is_complete: false,
        }
    }
    pub fn update<T: Animate>(&mut self, target: &mut T, delta_time: u32) {
        if self.is_complete {
            return;
        }

        self.elapsed += delta_time;
        let progress = (self.elapsed as f64 / self.duration as f64).min(1.0);

        match self.transformation {
            Transformation::Rotation { start, end } => {
                let current_rotation = lerp(start, end, progress);

                tracef!("Rotating: {}", current_rotation);

                target.set_rotation(current_rotation);
            }
        }

        if self.elapsed >= self.duration {
            self.is_complete = true;
        }
    }
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
}
fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}
