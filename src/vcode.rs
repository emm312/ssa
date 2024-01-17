use std::{fmt::Display, collections::HashMap};

use crate::{
    ir::{Instruction, Linkage, Terminator},
    regalloc::{VReg, Regalloc},
};

pub trait InstrSelector {
    type Instr: VCodeInstr;
    fn select(&mut self, gen: &mut VCodeGenerator<Self::Instr>, instr: &Instruction);
    fn select_terminator(&mut self, gen: &mut VCodeGenerator<Self::Instr>, term: &Terminator);
}

pub trait VCodeInstr {
    fn get_usable_regs() -> &'static [VReg];
    fn collect_registers(&self, regalloc: &mut impl Regalloc);
    fn apply_allocs(&mut self, allocs: &HashMap<VReg, VReg>);
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
    current_func: Option<usize>,
    current_block: Option<usize>,
    vreg_count: usize,
}

impl<I: VCodeInstr> Default for VCodeGenerator<I> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: VCodeInstr> VCodeGenerator<I> {
    pub fn new() -> VCodeGenerator<I> {
        VCodeGenerator {
            vcode: VCode { functions: vec![] },
            current_func: None,
            current_block: None,
            vreg_count: 0,
        }
    }
    pub fn push_vreg(&mut self) -> VReg {
        let vreg = VReg::Virtual(self.vreg_count);
        self.vreg_count += 1;
        vreg
    }
    pub fn push_instr(&mut self, instr: I) {
        self.vcode
            .functions
            .get_mut(self.current_func.unwrap())
            .unwrap()
            .instrs
            .get_mut(self.current_block.unwrap())
            .unwrap()
            .instrs
            .push(instr);
    }
    pub fn push_block(&mut self) -> usize {
        let func = self
            .vcode
            .functions
            .get_mut(self.current_func.unwrap())
            .unwrap();
        func.instrs.push(LabelledInstructions { instrs: vec![] });
        func.instrs.len() - 1
    }
    pub fn push_function(&mut self, name: &str, linkage: Linkage, arg_count: usize) -> usize {
        self.vcode.functions.push(VCodeFunction {
            name: name.to_string(),
            instrs: vec![],
            linkage,
            arg_count,
        });
        self.vcode.functions.len() - 1
    }
    pub fn switch_to_func(&mut self, id: usize) {
        self.current_func = Some(id);
    }
    pub fn switch_to_block(&mut self, id: usize) {
        self.current_block = Some(id);
    }
    pub fn build(self) -> VCode<I> {
        self.vcode
    }
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
