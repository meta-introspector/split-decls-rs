// Generated macro for remaining_files (function)
macro_rules! Depcrate_unix_linux_systemremaining_files {
() => {
// Module: crate::unix::linux::system
// Provides: {"remaining_files"}
// Dependencies: {}
pub (crate) fn remaining_files () -> & 'static AtomicIsize { static REMAINING_FILES : OnceLock < AtomicIsize > = OnceLock :: new () ; REMAINING_FILES . get_or_init (| | unsafe { let Some (mut limits) = getrlimit () else { return AtomicIsize :: new (1024 / 2) ; } ; let current = limits . rlim_cur ; limits . rlim_cur = limits . rlim_max ; AtomicIsize :: new (if libc :: setrlimit (libc :: RLIMIT_NOFILE , & limits) == 0 { limits . rlim_cur / 2 } else { current / 2 } as _) }) }
};
}
