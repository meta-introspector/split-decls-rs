// Generated macro for funmap (function)
macro_rules! Depcrate_callfunmap {
() => {
// Module: crate::call
// Provides: {"funmap"}
// Dependencies: {}
# [doc = " Unmap whole (or partial) continous memory-mapped files"] pub unsafe fn funmap (addr : usize , len : usize) -> Result < usize > { syscall2 (SYS_FUNMAP , addr , len) }
};
}
