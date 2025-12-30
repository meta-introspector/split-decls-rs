// Generated macro for symlink (function)
macro_rules! Depcrate_fs_abssymlink {
() => {
// Module: crate::fs::abs
// Provides: {"symlink"}
// Dependencies: {}
# [doc = " `symlink(old_path, new_path)`—Creates a symlink."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/symlink.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/symlink.2.html"] # [inline] pub fn symlink < P : path :: Arg , Q : path :: Arg > (old_path : P , new_path : Q) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | backend :: fs :: syscalls :: symlink (old_path , new_path)) }) }
};
}
