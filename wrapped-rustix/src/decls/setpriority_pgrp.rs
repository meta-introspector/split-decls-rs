macro_rules! deps {
    () => {
        Pid!();
        Result!();
    };
}

macro_rules! setpriority_pgrp {
    () => {
        deps!();
        # [doc = " `setpriority(PRIO_PGRP, pgid)`—Get the scheduling priority of the given"] # [doc = " process group."] # [doc = ""] # [doc = " A `pgid` of `None` means the process group of the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setpriority.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setpriority.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] # [inline] # [doc (alias = "setpriority")] pub fn setpriority_pgrp (pgid : Option < Pid > , priority : i32) -> io :: Result < () > { backend :: process :: syscalls :: setpriority_pgrp (pgid , priority) }
    };
}

setpriority_pgrp!()