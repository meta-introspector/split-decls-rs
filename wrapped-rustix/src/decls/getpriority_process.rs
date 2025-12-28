macro_rules! deps {
    () => {
        Result!();
        Pid!();
    };
}

macro_rules! getpriority_process {
    () => {
        deps!();
        # [doc = " `getpriority(PRIO_PROCESS, pid)`—Get the scheduling priority of the given"] # [doc = " process."] # [doc = ""] # [doc = " A `pid` of `None` means the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpriority.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getpriority.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] # [inline] # [doc (alias = "getpriority")] pub fn getpriority_process (pid : Option < Pid >) -> io :: Result < i32 > { backend :: process :: syscalls :: getpriority_process (pid) }
    };
}

getpriority_process!()