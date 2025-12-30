// Generated macro for dup (function)
macro_rules! Depcrate_calldup {
() => {
// Module: crate::call
// Provides: {"dup"}
// Dependencies: {}
# [doc = " Copy and transform a file descriptor"] pub fn dup (fd : usize , buf : & [u8]) -> Result < usize > { unsafe { syscall3 (SYS_DUP , fd , buf . as_ptr () as usize , buf . len ()) } }
};
}
