// Generated macro for sched_yield (function)
macro_rules! Depcrate_lib_generatedsched_yield {
() => {
// Module: crate::lib_generated
// Provides: {"sched_yield"}
// Dependencies: {}
# [doc = " Temporarily yield execution of the calling thread."] # [doc = " Note: This is similar to `sched_yield` in POSIX."] pub unsafe fn sched_yield () -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: sched_yield () ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
