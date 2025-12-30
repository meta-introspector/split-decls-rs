// Generated macro for sync (function)
macro_rules! Depcrate_fs_syncsync {
() => {
// Module: crate::fs::sync
// Provides: {"sync"}
// Dependencies: {}
# [doc = " `sync`—Flush cached filesystem data for all filesystems."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sync.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/sync.2.html"] # [inline] pub fn sync () { backend :: fs :: syscalls :: sync () ; }
};
}
