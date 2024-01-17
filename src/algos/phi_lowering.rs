use crate::ir::{Algo, Module};

pub fn lower_phis(module: &mut Module) {
    // Modules without critical edge splitting will have program semantics changed if phis are removed
    assert!(module.algos_run.contains(&Algo::CriticalEdgeSplitting));

    //for func in module.functions {
    //
    //}
}
