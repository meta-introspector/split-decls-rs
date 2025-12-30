// Generated macro for impl_1939 (impl)
macro_rules! Depcrate_thread_prctlimpl_1939 {
() => {
// Module: crate::thread::prctl
// Provides: {"impl_1939"}
// Dependencies: {}
impl TryFrom < u32 > for CoreSchedulingScope { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_SCHED_CORE_SCOPE_THREAD => Ok (Self :: Thread) , PR_SCHED_CORE_SCOPE_THREAD_GROUP => Ok (Self :: ThreadGroup) , PR_SCHED_CORE_SCOPE_PROCESS_GROUP => Ok (Self :: ProcessGroup) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
