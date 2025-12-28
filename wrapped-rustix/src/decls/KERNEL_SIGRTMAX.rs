macro_rules! KERNEL_SIGRTMAX {
    () => {
        # [doc = " `SIGRTMAX`—The last of the raw OS “real-time” signal range."] # [doc = ""] # [doc = " This is the raw `SIGRTMAX` value from the OS, which is not the same as the"] # [doc = " `SIGRTMAX` macro provided by libc. Don't use this unless you know your code"] # [doc = " won't share a process with a libc (perhaps because you yourself are"] # [doc = " implementing a libc)."] pub const KERNEL_SIGRTMAX : i32 = { # [cfg (not (any (target_arch = "arm" , target_arch = "s390x" , target_arch = "x86" , target_arch = "x86_64" ,)))] { linux_raw_sys :: general :: SIGRTMAX as i32 } # [cfg (any (target_arch = "arm" , target_arch = "s390x" , target_arch = "x86" , target_arch = "x86_64" ,))] { linux_raw_sys :: general :: _NSIG as i32 } } ;
    };
}

KERNEL_SIGRTMAX!();