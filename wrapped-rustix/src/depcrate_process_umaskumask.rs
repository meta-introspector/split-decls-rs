// Generated macro for umask (function)
macro_rules! Depcrate_process_umaskumask {
() => {
// Module: crate::process::umask
// Provides: {"umask"}
// Dependencies: {}
# [doc = " `umask(mask)`—Set the process file creation mask."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/umask.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/umask.2.html"] # [cfg (feature = "fs")] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] # [inline] pub fn umask (mask : Mode) -> Mode { backend :: process :: syscalls :: umask (mask) }
};
}
