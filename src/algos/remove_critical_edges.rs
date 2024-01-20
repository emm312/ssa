use crate::ir::{Algo, BasicBlock, BlockId, Module, Terminator};

pub fn remove_critical_edges(module: &mut Module) {
    module.algos_run.push(Algo::CriticalEdgeSplitting);
    for func in module.functions.iter_mut() {
        let mut to_insert = Vec::new();
        let blocks = func.blocks.clone();
        for (id, block) in blocks.iter().enumerate() {
            if let Terminator::Branch(_, bb_1, bb_2) = block.terminator {
                if blocks[bb_1.0].preds.len() > 1 {
                    let bb = BasicBlock {
                        instructions: vec![],
                        preds: vec![BlockId(id)],
                        terminator: Terminator::Jump(bb_1),
                        id: blocks.len() + to_insert.len(),
                        par_moves: blocks[bb_1.0].par_moves.clone(),
                    };
                    *func.blocks[bb_1.0]
                        .preds
                        .iter_mut()
                        .find(|x| **x == BlockId(block.id))
                        .unwrap() = BlockId(bb.id);
                    if let Terminator::Branch(_, ref mut bb, _) = func.blocks[id].terminator {
                        *bb = BlockId(blocks.len() + to_insert.len())
                    };
                    to_insert.push(bb);
                }
                if blocks[bb_2.0].preds.len() > 1 {
                    let bb = BasicBlock {
                        instructions: vec![],
                        preds: vec![BlockId(block.id)],
                        terminator: Terminator::Jump(bb_2),
                        id: blocks.len() + to_insert.len(),
                        par_moves: blocks[bb_2.0].par_moves.clone(),
                    };
                    *func.blocks[bb_2.0]
                        .preds
                        .iter_mut()
                        .find(|x| **x == BlockId(block.id))
                        .unwrap() = BlockId(bb.id);
                    if let Terminator::Branch(_, _, ref mut bb) = func.blocks[id].terminator {
                        *bb = BlockId(blocks.len() + to_insert.len())
                    };
                    to_insert.push(bb);
                }
            }
        }
        func.blocks.append(&mut to_insert)
    }
}
