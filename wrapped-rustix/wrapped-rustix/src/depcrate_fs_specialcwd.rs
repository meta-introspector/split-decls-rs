// Generated macro for CWD (const)
macro_rules! Depcrate_fs_specialCWD {
() => {
// Module: crate::fs::special
// Provides: {"CWD"}
// Dependencies: {}
# [doc = " `AT_FDCWD`—A handle representing the current working directory."] # [doc = ""] # [doc = " This is a file descriptor which refers to the process current directory"] # [doc = " which can be used as the directory argument in `*at` functions such as"] # [doc = " [`openat`]."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = ""] # [doc = " [`openat`]: crate::fs::openat"] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/fcntl.h.html"] # [cfg (not (target_os = "horizon"))] # [doc (alias = "AT_FDCWD")] pub const CWD : BorrowedFd < 'static > = unsafe { BorrowedFd :: < 'static > :: borrow_raw (c :: AT_FDCWD as RawFd) } ;
};
}
