// Generated macro for impl_518 (impl)
macro_rules! Depcrate_unix_apple_processimpl_518 {
() => {
// Module: crate::unix::apple::process
// Provides: {"impl_518"}
// Dependencies: {}
impl From < i32 > for ThreadStatus { fn from (status : i32) -> ThreadStatus { match status { libc :: TH_STATE_RUNNING => ThreadStatus :: Running , libc :: TH_STATE_STOPPED => ThreadStatus :: Stopped , libc :: TH_STATE_WAITING => ThreadStatus :: Waiting , libc :: TH_STATE_UNINTERRUPTIBLE => ThreadStatus :: Uninterruptible , libc :: TH_STATE_HALTED => ThreadStatus :: Halted , x => ThreadStatus :: Unknown (x) , } } }
};
}
