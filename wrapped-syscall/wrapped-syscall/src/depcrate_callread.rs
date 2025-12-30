// Generated macro for read (function)
macro_rules! Depcrate_callread {
() => {
// Module: crate::call
// Provides: {"read"}
// Dependencies: {}
# [doc = " Read from a file descriptor into a buffer"] pub fn read (fd : usize , buf : & mut [u8]) -> Result < usize > { unsafe { syscall3 (SYS_READ , fd , buf . as_mut_ptr () as usize , buf . len ()) } }
};
}
