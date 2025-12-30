// Generated macro for WaitIdStatus (struct)
macro_rules! Depcrate_process_waitWaitIdStatus {
() => {
// Module: crate::process::wait
// Provides: {"WaitIdStatus"}
// Dependencies: {}
# [doc = " The status of a process after calling [`waitid`]."] # [derive (Clone , Copy)] # [repr (transparent)] # [cfg (not (any (target_os = "horizon" , target_os = "openbsd" , target_os = "redox" , target_os = "wasi")))] pub struct WaitIdStatus (pub (crate) backend :: c :: siginfo_t) ;
};
}
