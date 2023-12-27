use super::{Regalloc, VReg};

pub struct GraphColouringRegalloc {}

impl Regalloc for GraphColouringRegalloc {
    fn force_same(&mut self, _a: VReg, _b: &[VReg]) {
        todo!()
    }
}
