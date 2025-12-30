// Generated macro for prctl_2args (function)
macro_rules! Depcrate_prctlprctl_2args {
() => {
// Module: crate::prctl
// Provides: {"prctl_2args"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn prctl_2args (option : c_int , arg2 : * mut c_void) -> io :: Result < c_int > { const NULL : * mut c_void = null_mut () ; syscalls :: prctl (option , arg2 , NULL , NULL , NULL) }
};
}
