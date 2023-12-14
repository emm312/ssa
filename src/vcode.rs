use std::fmt::Display;

use crate::{ir::{Instruction, Linkage, Module}, regalloc::VReg};

pub trait InstrSelector {
    type Instr: VCodeInstr;
    fn select(gen: &mut VCodeGenerator<Self::Instr>, instr: &Instruction) -> Self::Instr;
}

pub trait VCodeInstr {
    fn get_usable_regs() -> Vec<VReg>;
}

pub struct VCodeFunction<I: VCodeInstr> {
    pub name: String,
    pub instrs: Vec<LabelledInstructions<I>>,
    pub linkage: Linkage,
    pub arg_count: usize, // index of all the args in the fn
}

pub struct LabelledInstructions<I: VCodeInstr> {
    pub instrs: Vec<I>,
}

pub enum LabelDest {
    // usize: index of the func in the module
    Function(usize),
    // usize: index of the block in the function
    Block(usize),
}

pub struct VCode<I: VCodeInstr> {
    pub functions: Vec<VCodeFunction<I>>,
}

pub struct VCodeGenerator<I: VCodeInstr> {
    vcode: VCode<I>,
}

impl<I: VCodeInstr> VCodeGenerator<I> {
    
}

impl<I: Display + VCodeInstr> Display for VCode<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for func in self.functions.iter() {
            writeln!(f, "{}:", func.name)?;
            for (i, instrs) in func.instrs.iter().enumerate() {
                writeln!(f, "  .L{}:", i)?;
                for instr in instrs.instrs.iter() {
                    writeln!(f, "    {}", instr)?;
                }
            }
        }
        Ok(())
    }
}

impl<I: Display + VCodeInstr> Display for LabelledInstructions<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instr in self.instrs.iter() {
            writeln!(f, "    {}", instr)?;
        }
        Ok(())
    }
}

impl Display for LabelDest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LabelDest::Function(id) => write!(f, ".L{}", id),
            LabelDest::Block(id) => write!(f, ".L{}", id),
        }
    }
}