use super::{Regalloc, VReg};

pub struct GraphColouringRegalloc {

}

impl Regalloc for GraphColouringRegalloc {
    fn force_same(&mut self, a: VReg, b: &[VReg]) {
        todo!()
    }
}
