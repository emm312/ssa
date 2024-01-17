use std::{collections::HashMap, fmt::Display};

use crate::vcode::VCodeInstr;

pub mod linear_scan;

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
    fn add_def(&mut self, reg: VReg);
    fn add_use(&mut self, reg: VReg);
    fn next_instr(&mut self);
    fn coalesce_move(&mut self, from: VReg, to: VReg);
    fn alloc_regs<I: VCodeInstr>(&self) -> HashMap<VReg, VReg>;
}

pub fn apply_alloc(reg: &mut VReg, allocs: &HashMap<VReg, VReg>) {
    if let Some(new_reg) = allocs.get(reg) {
        *reg = *new_reg;
    }
}