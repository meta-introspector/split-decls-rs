// Generated macro for lseek (function)
macro_rules! Depcrate_calllseek {
() => {
// Module: crate::call
// Provides: {"lseek"}
// Dependencies: {}
# [doc = " Seek to `offset` bytes in a file descriptor"] pub fn lseek (fd : usize , offset : isize , whence : usize) -> Result < usize > { unsafe { syscall3 (SYS_LSEEK , fd , offset as usize , whence) } }
};
}
