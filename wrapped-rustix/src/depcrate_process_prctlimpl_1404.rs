// Generated macro for impl_1404 (impl)
macro_rules! Depcrate_process_prctlimpl_1404 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1404"}
// Dependencies: {}
impl TryFrom < u32 > for TimeStampCounterReadability { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_TSC_ENABLE => Ok (Self :: Readable) , PR_TSC_SIGSEGV => Ok (Self :: RaiseSIGSEGV) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
