// Generated macro for fstatvfs (function)
macro_rules! Depcrate_callfstatvfs {
() => {
// Module: crate::call
// Provides: {"fstatvfs"}
// Dependencies: {}
# [doc = " Get metadata about a filesystem"] pub fn fstatvfs (fd : usize , stat : & mut StatVfs) -> Result < usize > { unsafe { syscall3 (SYS_FSTATVFS , fd , stat as * mut StatVfs as usize , mem :: size_of :: < StatVfs > () ,) } }
};
}
