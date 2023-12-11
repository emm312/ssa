use crate::ir::Module;

pub fn delete(module: &mut Module, dels: Vec<Vec<Vec<bool>>>) {
    for (func_id, func) in (module.functions).iter_mut().enumerate() {
        for (block_id, block) in func.blocks.clone().iter().enumerate() {
            let mut new_instrs = Vec::new();
            for (instr_id, instr) in block.instructions.iter().enumerate() {
                if !dels[func_id][block_id][instr_id] {
                    new_instrs.push(instr.clone());
                }
            }
            func.blocks[block_id].instructions = new_instrs;
        }
    }
}
