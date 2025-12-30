// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_process_prctlimpl_1387 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1387"}
// Dependencies: {}
impl TryFrom < i32 > for TimingMethod { type Error = io :: Errno ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { PR_TIMING_STATISTICAL => Ok (Self :: Statistical) , PR_TIMING_TIMESTAMP => Ok (Self :: TimeStamp) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
