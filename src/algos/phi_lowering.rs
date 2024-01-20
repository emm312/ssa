use crate::{
    algos::delete_instructions::delete,
    ir::{Algo, Module, Operation},
};

pub fn lower_phis(module: &mut Module) {
    // Modules without critical edge splitting will have program semantics changed if phis are removed
    assert!(module.algos_run.contains(&Algo::CriticalEdgeSplitting));
    module.algos_run.push(Algo::PhiLowering);
    // Function: V: BB: I -> should delete instruction
    let mut func_dels: Vec<Vec<Vec<bool>>> = Vec::new();

    for (func_id, func) in module.functions.iter_mut().enumerate() {
        func_dels.push(Vec::new());
        for (block_id, block) in func.blocks.clone().into_iter().enumerate() {
            func_dels[func_id].push(Vec::new());
            for (instr_id, instr) in block.instructions.into_iter().enumerate() {
                func_dels[func_id][block_id].push(false);
                match &instr.operation {
                    Operation::Phi(defs) => {
                        for val in defs {
                            let target_block = func.values[val.0].owner;
                            func.blocks[target_block.0]
                                .par_moves
                                .push((instr.yielded.unwrap(), val.clone()))
                        }
                        func_dels[func_id][block_id][instr_id] = true;
                    }
                    _ => continue,
                }
            }
        }
    }
    delete(module, func_dels);
}
