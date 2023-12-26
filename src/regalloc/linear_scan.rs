use super::Regalloc;

pub struct LinearScanRegAlloc {
}

impl Regalloc for LinearScanRegAlloc {
    fn force_same(&mut self, a: super::VReg, b: &[super::VReg]) {
        todo!()
    }
}