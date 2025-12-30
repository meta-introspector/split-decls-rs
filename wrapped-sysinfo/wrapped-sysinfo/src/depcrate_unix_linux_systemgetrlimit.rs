// Generated macro for getrlimit (function)
macro_rules! Depcrate_unix_linux_systemgetrlimit {
() => {
// Module: crate::unix::linux::system
// Provides: {"getrlimit"}
// Dependencies: {}
unsafe fn getrlimit () -> Option < libc :: rlimit > { let mut limits = libc :: rlimit { rlim_cur : 0 , rlim_max : 0 , } ; if unsafe { libc :: getrlimit (libc :: RLIMIT_NOFILE , & mut limits) } != 0 { None } else { Some (limits) } }
};
}
