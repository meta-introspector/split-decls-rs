// Generated macro for fpath (function)
macro_rules! Depcrate_callfpath {
() => {
// Module: crate::call
// Provides: {"fpath"}
// Dependencies: {}
# [doc = " Retrieve the canonical path of a file"] pub fn fpath (fd : usize , buf : & mut [u8]) -> Result < usize > { unsafe { syscall3 (SYS_FPATH , fd , buf . as_mut_ptr () as usize , buf . len ()) } }
};
}
