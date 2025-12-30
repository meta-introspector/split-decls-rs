// Generated macro for fsync (function)
macro_rules! Depcrate_callfsync {
() => {
// Module: crate::call
// Provides: {"fsync"}
// Dependencies: {}
# [doc = " Sync a file descriptor to its underlying medium"] pub fn fsync (fd : usize) -> Result < usize > { unsafe { syscall1 (SYS_FSYNC , fd) } }
};
}
