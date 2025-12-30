// Generated macro for impl_1416 (impl)
macro_rules! Depcrate_process_prctlimpl_1416 {
() => {
// Module: crate::process::prctl
// Provides: {"impl_1416"}
// Dependencies: {}
impl TryFrom < u32 > for MachineCheckMemoryCorruptionKillPolicy { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_MCE_KILL_LATE => Ok (Self :: Late) , PR_MCE_KILL_EARLY => Ok (Self :: Early) , PR_MCE_KILL_DEFAULT => Ok (Self :: Default) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
