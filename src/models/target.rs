use crate::models::base::Base;

#[derive(Copy, Clone)]
pub struct Target {
    pub base: Base,
    pub required_bits: u32,
}

impl Target {
    pub fn new(base: Base, required_bits: u32) -> Target {
        return Target {base, required_bits};
    }
}