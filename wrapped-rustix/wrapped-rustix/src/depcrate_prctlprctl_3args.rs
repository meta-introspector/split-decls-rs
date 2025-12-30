// Generated macro for prctl_3args (function)
macro_rules! Depcrate_prctlprctl_3args {
() => {
// Module: crate::prctl
// Provides: {"prctl_3args"}
// Dependencies: {}
# [inline] pub (crate) unsafe fn prctl_3args (option : c_int , arg2 : * mut c_void , arg3 : * mut c_void ,) -> io :: Result < c_int > { syscalls :: prctl (option , arg2 , arg3 , null_mut () , null_mut ()) }
};
}
