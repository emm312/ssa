use crate::ir::{AnalysisStage, Module};

pub fn lower(module: &mut Module) {
    module.analysis_stage = AnalysisStage::LoweredToSSA;
    for function in module.functions.iter() {}
}
