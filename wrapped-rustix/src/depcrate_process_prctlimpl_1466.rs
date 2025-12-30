// Generated macro for impl_1466 (impl)
macro_rules! Depcrate_process_prctlimpl_1466 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1466"}
// Dependencies: {}
impl TryFrom < u32 > for SpeculationFeature { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_SPEC_STORE_BYPASS => Ok (Self :: SpeculativeStoreBypass) , PR_SPEC_INDIRECT_BRANCH => Ok (Self :: IndirectBranchSpeculation) , PR_SPEC_L1D_FLUSH => Ok (Self :: FlushL1DCacheOnContextSwitchOutOfTask) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
