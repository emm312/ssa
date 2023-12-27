use super::Regalloc;

pub struct LinearScanRegAlloc {}

impl Regalloc for LinearScanRegAlloc {
    fn force_same(&mut self, _a: super::VReg, _b: &[super::VReg]) {
        todo!()
    }
}
