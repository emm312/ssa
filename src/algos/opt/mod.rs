use crate::ir::Module;

pub mod constant_folding;

pub trait OptPass {
    fn run(&mut self, module: &mut Module);
}
