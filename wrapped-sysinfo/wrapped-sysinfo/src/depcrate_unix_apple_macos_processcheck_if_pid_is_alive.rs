// Generated macro for check_if_pid_is_alive (function)
macro_rules! Depcrate_unix_apple_macos_processcheck_if_pid_is_alive {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"check_if_pid_is_alive"}
// Dependencies: {}
# [inline] fn check_if_pid_is_alive (pid : Pid , check_if_alive : bool) -> bool { if ! check_if_alive { return true ; } unsafe { if kill (pid . 0 , 0) == 0 { return true ; } let errno = crate :: unix :: libc_errno () ; ! errno . is_null () && * errno != libc :: ESRCH } }
};
}
