// Generated macro for mprotect (function)
macro_rules! Depcrate_callmprotect {
() => {
// Module: crate::call
// Provides: {"mprotect"}
// Dependencies: {}
# [doc = " Change mapping flags"] pub unsafe fn mprotect (addr : usize , size : usize , flags : MapFlags) -> Result < usize > { syscall3 (SYS_MPROTECT , addr , size , flags . bits ()) }
};
}
