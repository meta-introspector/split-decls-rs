macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! setpriority_process {
    () => {
        deps!();
        # [doc = " `setpriority(PRIO_PROCESS, pid)`—Get the scheduling priority of the given"] # [doc = " process."] # [doc = ""] # [doc = " A `pid` of `None` means the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setpriority.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setpriority.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] # [inline] # [doc (alias = "setpriority")] pub fn setpriority_process (pid : Option < Pid > , priority : i32) -> io :: Result < () > { backend :: process :: syscalls :: setpriority_process (pid , priority) }
    };
}

setpriority_process!()