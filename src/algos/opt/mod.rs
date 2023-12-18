use crate::ir::Module;

pub mod dead_instr_elim;

pub trait OptPass {
    fn run(&mut self, module: &mut Module);
}