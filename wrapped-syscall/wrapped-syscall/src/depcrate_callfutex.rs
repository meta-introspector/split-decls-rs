// Generated macro for futex (function)
macro_rules! Depcrate_callfutex {
() => {
// Module: crate::call
// Provides: {"futex"}
// Dependencies: {}
# [doc = " Fast userspace mutex"] pub unsafe fn futex (addr : * mut i32 , op : usize , val : i32 , val2 : usize , addr2 : * mut i32 ,) -> Result < usize > { syscall5 (SYS_FUTEX , addr as usize , op , (val as isize) as usize , val2 , addr2 as usize ,) }
};
}
