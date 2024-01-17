use std::collections::HashMap;

use crate::ir::{
    Algo, BasicBlock, BlockId, Function, Instruction, Module, Operation, ValueId, VariableId,
};

use super::delete_instructions::delete;

/// Gets rid of all `load` and `store` instructions and replaces them with values and Φ functions.
///
/// notes on impl:
/// if no def in the current blk:
///  - recurse into preds and find a def for each one
///  - all bbs have a last definition
pub fn lower(module: &mut Module) {
    module.algos_run.push(Algo::PhiLowering);
    // Function: V: BB: I -> should delete instruction
    let mut func_dels: Vec<Vec<Vec<bool>>> = Vec::new();
    for (func_id, func) in (module.functions).iter_mut().enumerate() {
        func_dels.push(Vec::new());
        for (block_id, block) in func.blocks.clone().iter().enumerate() {
            func_dels[func_id].push(Vec::new());
            let mut last_var_defs = HashMap::new();
            for (instr_id, instr) in block.instructions.iter().enumerate() {
                func_dels[func_id][block_id].push(false);
                match instr.operation {
                    Operation::LoadVar(var) => {
                        if let Some(val) = last_var_defs.get(&var) {
                            // mark for deletion
                            func_dels[func_id][block_id][instr_id] = true;
                            // replace uses of the load with the val
                            func.replace_children_with(instr.yielded.unwrap(), *val);
                        } else {
                            // backtrack through preds for a def
                            let defs = find_defs_in_preds(
                                func,
                                BlockId(block_id),
                                var,
                                BlockId(block_id),
                                instr.yielded.unwrap(),
                            );
                            func.replace_instruction(
                                BlockId(block_id),
                                instr_id,
                                Instruction {
                                    yielded: instr.yielded,
                                    operation: Operation::Phi(defs),
                                },
                            )
                        }
                    }
                    Operation::StoreVar(to, val) => {
                        last_var_defs.insert(to, val);
                        func_dels[func_id][block_id][instr_id] = true;
                    }
                    _ => {}
                }
            }
        }
    }
    delete(module, func_dels);
    remove_singleelem_phis(module);
}

pub fn remove_singleelem_phis(module: &mut Module) {
    let mut dels = Vec::new();
    for (func_id, func) in module.functions.iter_mut().enumerate() {
        dels.push(Vec::new());
        for (block_id, block) in func.blocks.clone().iter().enumerate() {
            dels[func_id].push(Vec::new());
            for (pos, instr) in block.instructions.iter().enumerate() {
                dels[func_id][block_id].push(false);
                if let Operation::Phi(ref vals) = instr.operation {
                    if vals.len() == 1 {
                        func.replace_children_with(instr.yielded.unwrap(), vals[0]);
                        // mark for deletion
                        dels[func_id][block_id][pos] = true;
                    }
                }
            }
        }
    }
    delete(module, dels);
}

fn find_last_def(bb: &BasicBlock, var: VariableId) -> Option<ValueId> {
    let rev_instrs = bb.instructions.iter().rev();
    for instr in rev_instrs {
        if let Operation::StoreVar(to, val) = instr.operation {
            if to == var {
                return Some(val);
            }
        }
    }
    None
}

fn find_defs_in_preds(
    func: &Function,
    block_id: BlockId,
    var: VariableId,
    stop_at: BlockId,
    to: ValueId,
) -> Vec<ValueId> {
    let mut defs = Vec::new();
    for pred in func.blocks[block_id.0].preds.clone() {
        if pred == stop_at {
            defs.push(to);
            return defs;
        }
        if let Some(val) = find_last_def(&func.blocks[pred.0], var) {
            defs.push(val);
        } else {
            defs.append(&mut find_defs_in_preds(func, pred, var, stop_at, to));
        }
    }
    defs
}
