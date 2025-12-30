// Generated macro for renameat (function)
macro_rules! Depcrate_fs_atrenameat {
() => {
// Module: crate::fs::at
// Provides: {"renameat"}
// Dependencies: {}
# [doc = " `renameat(old_dirfd, old_path, new_dirfd, new_path)`—Renames a file or"] # [doc = " directory."] # [doc = ""] # [doc = " See [`renameat_with`] to pass additional flags."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/renameat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/renameat.2.html"] # [inline] pub fn renameat < P : path :: Arg , Q : path :: Arg , PFd : AsFd , QFd : AsFd > (old_dirfd : PFd , old_path : P , new_dirfd : QFd , new_path : Q ,) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | { backend :: fs :: syscalls :: renameat (old_dirfd . as_fd () , old_path , new_dirfd . as_fd () , new_path ,) }) }) }
};
}
