// Generated macro for rename (function)
macro_rules! Depcrate_fs_absrename {
() => {
// Module: crate::fs::abs
// Provides: {"rename"}
// Dependencies: {}
# [doc = " `rename(old_path, new_path)`—Renames a file or directory."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/rename.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/rename.2.html"] # [inline] pub fn rename < P : path :: Arg , Q : path :: Arg > (old_path : P , new_path : Q) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | backend :: fs :: syscalls :: rename (old_path , new_path)) }) }
};
}
