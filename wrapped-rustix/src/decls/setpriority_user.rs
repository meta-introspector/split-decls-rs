macro_rules! deps {
    () => {
        Result!();
        Uid!();
    };
}

macro_rules! setpriority_user {
    () => {
        deps!();
        # [doc = " `setpriority(PRIO_USER, uid)`—Get the scheduling priority of the given"] # [doc = " user."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setpriority.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setpriority.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] # [inline] # [doc (alias = "setpriority")] pub fn setpriority_user (uid : Uid , priority : i32) -> io :: Result < () > { backend :: process :: syscalls :: setpriority_user (uid , priority) }
    };
}

setpriority_user!();