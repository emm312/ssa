use regalloc2::{Block, Function as RegAllocFunction, InstRange, VReg, RegClass};

use crate::ir::{BlockId, Function, Terminator, Operation, Value, Type};

impl Into<Block> for BlockId {
    fn into(self) -> Block {
        Block(self.0 as u32)
    }
}

impl Into<BlockId> for Block {
    fn into(self) -> BlockId {
        BlockId(self.0 as usize)
    }
}

impl Into<(usize, RegClass)> for Value {
    fn into(self) -> (usize, RegClass) {
        match self.ty {
            Type::Integer(size, _) => (size as usize, RegClass::Int),
            Type::Pointer(_) => (32, RegClass::Int),
            Type::Void => (0, RegClass::Int),
        }
    }
}

impl RegAllocFunction for Function {
    fn num_blocks(&self) -> usize {
        self.blocks.len()
    }
    fn num_insts(&self) -> usize {
        self.blocks.iter().map(|b| b.instructions.len()).sum()
    }
    fn entry_block(&self) -> regalloc2::Block {
        Block(0)
    }
    fn block_insns(&self, block: Block) -> regalloc2::InstRange {
        let block = &self.blocks[block.0 as usize];
    }
    fn block_succs(&self, block: Block) -> &[Block] {
        let block = &self.blocks[block.0 as usize];
        match block.terminator {
            Terminator::Branch(_, a, b) => &[a.into(), b.into()],
            Terminator::Jump(a) => &[a.into()],
            Terminator::Return(_) => &[],
            Terminator::NoTerm => &[],
        }
    }
    fn block_preds(&self, block: Block) -> &[Block] {
        self.blocks[block.0 as usize]
            .preds
            .iter()
            .map(|elem| (*elem).into())
            .collect::<Vec<Block>>()
            .as_slice()
    }
    fn block_params(&self, block: Block) -> &[regalloc2::VReg] {
        &self.blocks[block.0 as usize]
            .instructions
            .iter()
            .filter(|elem| matches!(elem.operation, Operation::Phi(_)))
            .map(|elem| elem.yielded.unwrap())
            .map(|elem| {
                let bb_elem = self.values[elem.0 as usize];
                let s: (usize, RegClass) = bb_elem.into();
                VReg::new(s.0, s.1)
            })
            .collect::<Vec<VReg>>()
            .as_slice()
    }
    fn is_ret(&self, insn: regalloc2::Inst) -> bool {
        
    }
}
