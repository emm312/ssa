use super::OptPass;

pub struct ConstantFolding {}

impl OptPass for ConstantFolding {
    fn run(&mut self, module: &mut crate::ir::Module) {}
}
