use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VReg {
    Virtual(usize),
    Real(usize),
    Spilled(usize),
}

impl Display for VReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VReg::Virtual(id) => write!(f, "v{}", id),
            VReg::Real(id) => write!(f, "r{}", id),
            VReg::Spilled(id) => write!(f, "[s{}]", id),
        }
    }
}

pub trait Regalloc {
    /// This function forces a set of registers to match a VReg::Real
    fn force_same(&mut self, a: VReg, b: &[VReg]);
}