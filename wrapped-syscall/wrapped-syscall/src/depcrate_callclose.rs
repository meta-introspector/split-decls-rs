// Generated macro for close (function)
macro_rules! Depcrate_callclose {
() => {
// Module: crate::call
// Provides: {"close"}
// Dependencies: {}
# [doc = " Close a file"] pub fn close (fd : usize) -> Result < usize > { unsafe { syscall1 (SYS_CLOSE , fd) } }
};
}
