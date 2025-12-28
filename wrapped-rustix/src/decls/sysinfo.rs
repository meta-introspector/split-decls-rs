macro_rules! sysinfo {
    () => {
        # [doc = " `sysinfo()`—Returns status information about the runtime OS."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/uname.2.html"] # [cfg (linux_kernel)] # [inline] pub fn sysinfo () -> Sysinfo { backend :: system :: syscalls :: sysinfo () }
    };
}

sysinfo!()