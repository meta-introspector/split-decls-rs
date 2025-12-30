// Generated macro for impl_1931 (impl)
macro_rules! Depcrate_thread_prctlimpl_1931 {
() => {
// Module: crate::thread::prctl
// Provides: {"impl_1931"}
// Dependencies: {}
impl TryFrom < u8 > for SysCallUserDispatchFastSwitch { type Error = io :: Errno ; fn try_from (value : u8) -> Result < Self , Self :: Error > { match value { SYSCALL_DISPATCH_FILTER_ALLOW => Ok (Self :: Allow) , SYSCALL_DISPATCH_FILTER_BLOCK => Ok (Self :: Block) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
