use crate::ir::{Algo, Module};

pub fn lower_par_moves(module: &mut Module) {
    assert!(module.algos_run.contains(&Algo::PhiLowering));
    module.algos_run.push(Algo::LowerParMoves);

    for (func_id, func) in module.functions.iter().enumerate() {
        for (block_id, block) in func.blocks.iter().enumerate() {
            block.par_moves.iter().for_each(|(from, to)| {
                
            })
        }
    }
}