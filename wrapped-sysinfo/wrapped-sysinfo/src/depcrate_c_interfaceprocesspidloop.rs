// Generated macro for ProcessPidLoop (type)
macro_rules! Depcrate_c_interfaceProcessPidLoop {
() => {
// Module: crate::c_interface
// Provides: {"ProcessPidLoop"}
// Dependencies: {}
# [doc = " Callback used by [`tasks`][crate::Process#method.tasks]."] pub type ProcessPidLoop = extern "C" fn (pid : PID , data : * mut c_void) -> bool ;
};
}
