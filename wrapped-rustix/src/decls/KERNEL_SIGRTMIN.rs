macro_rules! KERNEL_SIGRTMIN {
    () => {
        # [doc = " `SIGRTMIN`—The start of the raw OS “real-time” signal range."] # [doc = ""] # [doc = " This is the raw `SIGRTMIN` value from the OS, which is not the same as the"] # [doc = " `SIGRTMIN` macro provided by libc. Don't use this unless you know your code"] # [doc = " won't share a process with a libc (perhaps because you yourself are"] # [doc = " implementing a libc)."] pub const KERNEL_SIGRTMIN : i32 = linux_raw_sys :: general :: SIGRTMIN as i32 ;
    };
}

KERNEL_SIGRTMIN!();