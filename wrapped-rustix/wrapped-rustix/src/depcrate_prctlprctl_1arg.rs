// Generated macro for prctl_1arg (function)
macro_rules! Depcrate_prctlprctl_1arg {
() => {
// Module: crate::prctl
// Provides: {"prctl_1arg"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn prctl_1arg (option : c_int) -> io :: Result < c_int > { const NULL : * mut c_void = null_mut () ; syscalls :: prctl (option , NULL , NULL , NULL , NULL) }
};
}
