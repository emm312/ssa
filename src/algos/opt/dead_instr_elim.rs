use crate::ir::Terminator;

use super::OptPass;

pub struct DeadInstrElim {}

impl OptPass for DeadInstrElim {
    fn run(&mut self, module: &mut crate::ir::Module) {
        for func in &module.functions {
            'val_loop: for (pos, val) in func.values.iter().enumerate() {
                if val.children.is_empty() {
                    for block in &func.blocks {
                        match block.terminator {
                            Terminator::Branch(id, ..) | Terminator::Return(id) => {
                                if pos == id.0 {
                                    break 'val_loop;
                                }
                            }
                            _ => {}
                        }
                    }
                    // TODO: delete assignment to the instr

                }
            }
        }
    }
}