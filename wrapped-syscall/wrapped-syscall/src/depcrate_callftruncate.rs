// Generated macro for ftruncate (function)
macro_rules! Depcrate_callftruncate {
() => {
// Module: crate::call
// Provides: {"ftruncate"}
// Dependencies: {}
# [doc = " Truncate or extend a file to a specified length"] pub fn ftruncate (fd : usize , len : usize) -> Result < usize > { unsafe { syscall2 (SYS_FTRUNCATE , fd , len) } }
};
}
