// Generated macro for ProcessLoop (type)
macro_rules! Depcrate_c_interfaceProcessLoop {
() => {
// Module: crate::c_interface
// Provides: {"ProcessLoop"}
// Dependencies: {}
# [doc = " Callback used by [`processes`][crate::System#method.processes]."] pub type ProcessLoop = extern "C" fn (pid : PID , process : CProcess , data : * mut c_void) -> bool ;
};
}
