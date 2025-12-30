// Generated macro for fstat (function)
macro_rules! Depcrate_callfstat {
() => {
// Module: crate::call
// Provides: {"fstat"}
// Dependencies: {}
# [doc = " Get metadata about a file"] pub fn fstat (fd : usize , stat : & mut Stat) -> Result < usize > { unsafe { syscall3 (SYS_FSTAT , fd , stat as * mut Stat as usize , mem :: size_of :: < Stat > () ,) } }
};
}
