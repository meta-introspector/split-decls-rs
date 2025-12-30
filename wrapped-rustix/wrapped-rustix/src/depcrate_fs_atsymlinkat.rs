// Generated macro for symlinkat (function)
macro_rules! Depcrate_fs_atsymlinkat {
() => {
// Module: crate::fs::at
// Provides: {"symlinkat"}
// Dependencies: {}
# [doc = " `symlinkat(old_path, new_dirfd, new_path)`—Creates a symlink."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/symlinkat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/symlinkat.2.html"] # [inline] pub fn symlinkat < P : path :: Arg , Q : path :: Arg , Fd : AsFd > (old_path : P , new_dirfd : Fd , new_path : Q ,) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | { backend :: fs :: syscalls :: symlinkat (old_path , new_dirfd . as_fd () , new_path) }) }) }
};
}
