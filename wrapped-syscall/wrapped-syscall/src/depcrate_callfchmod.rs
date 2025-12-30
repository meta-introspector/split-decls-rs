// Generated macro for fchmod (function)
macro_rules! Depcrate_callfchmod {
() => {
// Module: crate::call
// Provides: {"fchmod"}
// Dependencies: {}
# [doc = " Change file permissions"] pub fn fchmod (fd : usize , mode : u16) -> Result < usize > { unsafe { syscall2 (SYS_FCHMOD , fd , mode as usize) } }
};
}
