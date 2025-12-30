// Generated macro for rmdir (function)
macro_rules! Depcrate_fs_absrmdir {
() => {
// Module: crate::fs::abs
// Provides: {"rmdir"}
// Dependencies: {}
# [doc = " `rmdir(path)`—Removes a directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/rmdir.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/rmdir.2.html"] # [inline] pub fn rmdir < P : path :: Arg > (path : P) -> io :: Result < () > { path . into_with_c_str (backend :: fs :: syscalls :: rmdir) }
};
}
