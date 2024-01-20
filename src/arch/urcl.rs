use std::fmt::Display;

use crate::{
    ir::{BinOp, Instruction, Operation, Terminator, ValueId},
    regalloc::{apply_alloc, VReg},
    vcode::{InstrSelector, LabelDest, VCodeGenerator, VCodeInstr},
};

pub const URCL_REG_ZR: usize = 0;
pub const URCL_REG_1: usize = 1;
pub const URCL_REG_2: usize = 2;
pub const URCL_REG_3: usize = 3;
pub const URCL_REG_4: usize = 4;
pub const URCL_REG_5: usize = 5;
pub const URCL_REG_6: usize = 6;
pub const URCL_REG_7: usize = 7;
pub const URCL_REG_8: usize = 8;

/// URCL DEFAULT CALLING CONV:
/// - r1: return value

pub enum UrclInstr {
    PhiPlaceholder {
        dst: VReg,
        ops: Vec<VReg>,
    },
    AluOp {
        op: UrclAluOp,
        dst: VReg,
        src1: VReg,
        src2: VReg,
    },
    Jmp {
        dst: LabelDest,
    },
    Beq {
        src1: VReg,
        dst: LabelDest,
    },
    Imm {
        dst: VReg,
        val: i64,
    },
    Mov {
        dst: VReg,
        src: VReg,
    },
    Cal {
        dst: LabelDest,
    },
    Ret,
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
    Rsh,
    Lsh,
    Ssete,
    Ssetne,
    Ssetl,
    Ssetle,
    Ssetg,
    Ssetge,
}

impl From<BinOp> for UrclAluOp {
    fn from(op: BinOp) -> Self {
        match op {
            BinOp::Add => UrclAluOp::Add,
            BinOp::Sub => UrclAluOp::Sub,
            BinOp::Mul => UrclAluOp::Mul,
            BinOp::Div => UrclAluOp::Div,
            BinOp::Mod => UrclAluOp::Mod,
            BinOp::And => UrclAluOp::And,
            BinOp::Or => UrclAluOp::Or,
            BinOp::Xor => UrclAluOp::Xor,
            BinOp::Eq => UrclAluOp::Ssete,
            BinOp::Ne => UrclAluOp::Ssetne,
            BinOp::Lt => UrclAluOp::Ssetl,
            BinOp::Le => UrclAluOp::Ssetle,
            BinOp::Gt => UrclAluOp::Ssetg,
            BinOp::Ge => UrclAluOp::Ssetge,
            BinOp::Shl => UrclAluOp::Lsh,
            BinOp::Shr => UrclAluOp::Rsh,
        }
    }
}

impl VCodeInstr for UrclInstr {
    fn get_usable_regs() -> &'static [VReg] {
        &[
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

    fn collect_registers(&self, regalloc: &mut impl crate::regalloc::Regalloc) {
        match self {
            Self::AluOp {
                dst, src1, src2, ..
            } => {
                regalloc.add_def(*dst);
                regalloc.add_use(*src1);
                regalloc.add_use(*src2);
            }
            Self::Jmp { .. } => (),
            Self::Beq { src1, .. } => {
                regalloc.add_use(*src1);
            }
            Self::Imm { dst, .. } => {
                regalloc.add_def(*dst);
            }
            Self::Mov { dst, src } => {
                regalloc.add_def(*dst);
                regalloc.add_use(*src);
                regalloc.coalesce_move(*src, *dst);
            }
            _ => (),
        }
    }

    fn apply_allocs(&mut self, allocs: &std::collections::HashMap<VReg, VReg>) {
        match self {
            Self::AluOp {
                dst, src1, src2, ..
            } => {
                apply_alloc(dst, allocs);
                apply_alloc(src1, allocs);
                apply_alloc(src2, allocs);
            }
            Self::Jmp { .. } => (),
            Self::Beq { src1, .. } => {
                apply_alloc(src1, allocs);
            }
            Self::Imm { dst, .. } => {
                apply_alloc(dst, allocs);
            }
            Self::Mov { dst, src } => {
                apply_alloc(dst, allocs);
                apply_alloc(src, allocs);
            }
            _ => (),
        }
    }
}

impl Display for UrclInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrclInstr::AluOp {
                op,
                dst,
                src1,
                src2,
            } => {
                write!(f, "{} {} {} {}", op, dst, src1, src2)
            }
            UrclInstr::Jmp { dst } => write!(f, "jmp {}", dst),
            UrclInstr::Imm { dst, val } => write!(f, "imm {} {}", dst, val),
            UrclInstr::Beq { src1, dst } => write!(f, "bgr {} {} 0", dst, src1),
            UrclInstr::Mov { dst, src } => write!(f, "mov {} {}", dst, src),
            UrclInstr::Cal { dst } => write!(f, "cal {}", dst),
            UrclInstr::Ret => write!(f, "ret"),
            UrclInstr::PhiPlaceholder { dst, ops } => write!(
                f,
                "phi {} {}",
                dst,
                ops.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
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
            UrclAluOp::Rsh => write!(f, "rsh"),
            UrclAluOp::Lsh => write!(f, "lsh"),
            UrclAluOp::Ssete => write!(f, "ssete"),
            UrclAluOp::Ssetne => write!(f, "ssetne"),
            UrclAluOp::Ssetl => write!(f, "ssetl"),
            UrclAluOp::Ssetle => write!(f, "ssetle"),
            UrclAluOp::Ssetg => write!(f, "ssetg"),
            UrclAluOp::Ssetge => write!(f, "ssetge"),
        }
    }
}

#[derive(Default)]
pub struct UrclSelector;

impl InstrSelector for UrclSelector {
    type Instr = UrclInstr;
    fn select(&mut self, gen: &mut VCodeGenerator<Self::Instr>, instr: &Instruction) {
        let dst = if let Some(val) = instr.yielded {
            self.get_vreg(val)
        } else {
            VReg::Real(URCL_REG_ZR)
        };

        match &instr.operation {
            Operation::BinOp(op, lhs, rhs) => {
                let src1 = self.get_vreg(*lhs);
                let src2 = self.get_vreg(*rhs);
                gen.push_instr(UrclInstr::AluOp {
                    op: (*op).into(),
                    dst,
                    src1,
                    src2,
                });
            }
            Operation::Integer(val) => {
                gen.push_instr(UrclInstr::Imm { dst, val: *val });
            }
            Operation::LoadVar(_) | Operation::StoreVar(..) => (), // THESE NEVER GET EXECUTED (removed in algos::lower_to_ssa::lower())
            Operation::Phi(vals) => {
                gen.push_instr(UrclInstr::PhiPlaceholder {
                    dst,
                    ops: vals.iter().map(|v| self.get_vreg(*v)).collect(),
                });
            }
            _ => todo!(),
        }
    }

    fn select_terminator(&mut self, gen: &mut VCodeGenerator<Self::Instr>, term: &Terminator) {
        match term {
            Terminator::Branch(val, t, f) => {
                gen.push_instr(UrclInstr::Beq {
                    src1: self.get_vreg(*val),
                    dst: LabelDest::Block(t.0),
                });
                gen.push_instr(UrclInstr::Jmp {
                    dst: LabelDest::Block(f.0),
                });
            }
            Terminator::Jump(l) => {
                gen.push_instr(UrclInstr::Jmp {
                    dst: LabelDest::Block(l.0),
                });
            }
            Terminator::Return(val) => {
                gen.push_instr(UrclInstr::Mov {
                    dst: VReg::Real(URCL_REG_1),
                    src: self.get_vreg(*val),
                });
                gen.push_instr(UrclInstr::Ret);
            }
            _ => todo!(),
        }
    }

    fn get_post_function_instructions(&mut self, gen: &mut VCodeGenerator<Self::Instr>) {
        
    }

    fn get_pre_function_instructions(&mut self, gen: &mut VCodeGenerator<Self::Instr>) {
        
    }
}

impl UrclSelector {
    #[inline]
    pub fn get_vreg(&self, val: ValueId) -> VReg {
        VReg::Virtual(val.0)
    }
}
