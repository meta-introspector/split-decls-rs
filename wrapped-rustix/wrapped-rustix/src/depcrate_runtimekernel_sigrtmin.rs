// Generated macro for KERNEL_SIGRTMIN (const)
macro_rules! Depcrate_runtimeKERNEL_SIGRTMIN {
() => {
// Module: crate::runtime
// Provides: {"KERNEL_SIGRTMIN"}
// Dependencies: {}
# [doc = " `SIGRTMIN`—The start of the raw OS “real-time” signal range."] # [doc = ""] # [doc = " This is the raw `SIGRTMIN` value from the OS, which is not the same as the"] # [doc = " `SIGRTMIN` macro provided by libc. Don't use this unless you know your code"] # [doc = " won't share a process with a libc (perhaps because you yourself are"] # [doc = " implementing a libc)."] pub const KERNEL_SIGRTMIN : i32 = linux_raw_sys :: general :: SIGRTMIN as i32 ;
};
}
