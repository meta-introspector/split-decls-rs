// Generated macro for dup2 (function)
macro_rules! Depcrate_calldup2 {
() => {
// Module: crate::call
// Provides: {"dup2"}
// Dependencies: {}
# [doc = " Copy and transform a file descriptor"] pub fn dup2 (fd : usize , newfd : usize , buf : & [u8]) -> Result < usize > { unsafe { syscall4 (SYS_DUP2 , fd , newfd , buf . as_ptr () as usize , buf . len ()) } }
};
}
