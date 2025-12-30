// Generated macro for getpriority_user (function)
macro_rules! Depcrate_process_prioritygetpriority_user {
() => {
// Module: crate::process::priority
// Provides: {"getpriority_user"}
// Dependencies: {}
# [doc = " `getpriority(PRIO_USER, uid)`—Get the scheduling priority of the given"] # [doc = " user."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getpriority.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/getpriority.2.html"] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/setpriority.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] # [inline] # [doc (alias = "getpriority")] pub fn getpriority_user (uid : Uid) -> io :: Result < i32 > { backend :: process :: syscalls :: getpriority_user (uid) }
};
}
