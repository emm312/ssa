use std::{collections::HashMap, ops::Range};

use crate::vcode::VCodeInstr;

use super::{Regalloc, VReg};

#[derive(Default)]
pub struct LinearScanRegAlloc {
    registers: Vec<RegAllocReg>,
    live_count: usize,
    spill_counter: usize,
}

pub struct RegAllocReg {
    live_range: Range<usize>,
    uses: usize,
    reg: VReg,
    try_to_coalesce_to: Option<VReg>
}

impl Regalloc for LinearScanRegAlloc {
    fn add_def(&mut self, reg: VReg) {
        if !matches!(reg, VReg::Virtual(_)) {
            return;
        }
        if let Some(reg) = self.registers.iter_mut().find(|e| e.reg == reg) {
            reg.live_range.end = self.live_count;
        } else {
            self.registers.push(RegAllocReg {
                live_range: self.live_count..self.live_count,
                uses: 0,
                reg,
                try_to_coalesce_to: None
            });
        }
    }
    fn add_use(&mut self, reg: VReg) {
        if !matches!(reg, VReg::Virtual(_)) {
            return;
        }
        if let Some(reg) = self.registers.iter_mut().find(|e| e.reg == reg) {
            reg.uses += 1;
            reg.live_range.end = self.live_count;
        } else {
            self.registers.push(RegAllocReg {
                live_range: self.live_count..self.live_count,
                uses: 1,
                reg,
                try_to_coalesce_to: None
            });
        }
    }
    fn next_instr(&mut self) {
        self.live_count += 1;
    }
    fn coalesce_move(&mut self, from: VReg, to: VReg) {
        if !(matches!(from, VReg::Virtual(_))) {
            return;
        }
        if let Some(reg) = self.find_reg(from) {
            reg.try_to_coalesce_to = Some(to);
        }
    }
    fn alloc_regs<I: VCodeInstr>(&self) -> HashMap<VReg, VReg> {
        let mut ret = HashMap::new();
        let mut reg_stack = I::get_usable_regs().to_vec();
        for i in 0..self.live_count {
            for reg in &self.registers {
                if reg.live_range.start == i {
                    if let Some(to) = reg_stack.pop() {
                        ret.insert(reg.reg, to);
                    } else {

                    }
                }
                if reg.live_range.end == i {
                    if let Some(VReg::Real(reg)) = ret.get(&reg.reg) {
                        reg_stack.push(VReg::Real(*reg));
                    }
                }
            }
        }
        println!("{:#?}", ret);
        ret
    }
}

impl LinearScanRegAlloc {
    fn find_reg(&mut self, reg: VReg) -> Option<&mut RegAllocReg> {
        self.registers.iter_mut().find(|e| e.reg == reg)
    }
}
