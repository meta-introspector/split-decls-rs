// Generated macro for fcntl (function)
macro_rules! Depcrate_callfcntl {
() => {
// Module: crate::call
// Provides: {"fcntl"}
// Dependencies: {}
# [doc = " Change file descriptor flags"] pub fn fcntl (fd : usize , cmd : usize , arg : usize) -> Result < usize > { unsafe { syscall3 (SYS_FCNTL , fd , cmd , arg) } }
};
}
