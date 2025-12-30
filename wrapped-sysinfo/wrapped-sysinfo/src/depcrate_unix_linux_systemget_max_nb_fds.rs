// Generated macro for get_max_nb_fds (function)
macro_rules! Depcrate_unix_linux_systemget_max_nb_fds {
() => {
// Module: crate::unix::linux::system
// Provides: {"get_max_nb_fds"}
// Dependencies: {}
pub (crate) fn get_max_nb_fds () -> usize { unsafe { let mut limits = libc :: rlimit { rlim_cur : 0 , rlim_max : 0 , } ; if libc :: getrlimit (libc :: RLIMIT_NOFILE , & mut limits) != 0 { 1024 / 2 } else { limits . rlim_max as usize / 2 } } }
};
}
