use smart_default::SmartDefault;

#[derive(Debug, SmartDefault)]
pub struct NumberEffect {
    points: u32,
    mult: u32,
    #[default = 1]
    multx: u32,
}

pub enum Effect {
    Numbers(NumberEffect),
}
