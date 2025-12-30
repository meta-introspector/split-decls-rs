// Generated macro for mkdir (function)
macro_rules! Depcrate_fs_absmkdir {
() => {
// Module: crate::fs::abs
// Provides: {"mkdir"}
// Dependencies: {}
# [doc = " `mkdir(path, mode)`—Creates a directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/mkdir.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mkdir.2.html"] # [inline] pub fn mkdir < P : path :: Arg > (path : P , mode : Mode) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: mkdir (path , mode)) }
};
}
