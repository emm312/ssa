use std::{fmt::Display, collections::HashMap};

use crate::{regalloc::VReg, vcode::{VCodeInstr, LabelDest, InstrSelector, VCodeGenerator}, ir::{Value, Instruction}};

pub const URCL_REG_1: usize = 1;
pub const URCL_REG_2: usize = 2;
pub const URCL_REG_3: usize = 3;
pub const URCL_REG_4: usize = 4;
pub const URCL_REG_5: usize = 5;
pub const URCL_REG_6: usize = 6;
pub const URCL_REG_7: usize = 7;
pub const URCL_REG_8: usize = 8;

pub enum UrclInstr {
    AluOp {
        op: UrclAluOp,
        dst: VReg,
        src1: VReg,
        src2: VReg,
    },
    Jmp {
        dst: LabelDest,
    },
}

pub enum UrclAluOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Neg,
}

impl VCodeInstr for UrclInstr {
    fn get_usable_regs() -> Vec<VReg> {
        vec![
            VReg::Real(URCL_REG_1),
            VReg::Real(URCL_REG_2),
            VReg::Real(URCL_REG_3),
            VReg::Real(URCL_REG_4),
            VReg::Real(URCL_REG_5),
            VReg::Real(URCL_REG_6),
            VReg::Real(URCL_REG_7),
            VReg::Real(URCL_REG_8),
        ]
    }
}

impl Display for UrclInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrclInstr::AluOp { op, dst, src1, src2 } => {
                write!(f, "{} {}, {}, {}", op, dst, src1, src2)
            }
            UrclInstr::Jmp { dst } => write!(f, "jmp {}", dst),
        }
    }
}

impl Display for UrclAluOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrclAluOp::Add => write!(f, "add"),
            UrclAluOp::Sub => write!(f, "sub"),
            UrclAluOp::Mul => write!(f, "mul"),
            UrclAluOp::Div => write!(f, "div"),
            UrclAluOp::Mod => write!(f, "mod"),
            UrclAluOp::And => write!(f, "and"),
            UrclAluOp::Or => write!(f, "or"),
            UrclAluOp::Xor => write!(f, "xor"),
            UrclAluOp::Not => write!(f, "not"),
            UrclAluOp::Neg => write!(f, "neg"),
        }
    }
}

#[derive(Default)]
pub struct UrclSelector {
    val_map: HashMap<Value, VReg>,
    vreg_idx: usize,
}

impl InstrSelector for UrclSelector {
    type Instr = UrclInstr;
    fn select(gen: &mut VCodeGenerator<Self::Instr>, instr: &Instruction) -> UrclInstr {
        todo!()
    }
}