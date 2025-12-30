// Generated macro for renameat_with (function)
macro_rules! Depcrate_fs_atrenameat_with {
() => {
// Module: crate::fs::at
// Provides: {"renameat_with"}
// Dependencies: {}
# [doc = " `renameat2(old_dirfd, old_path, new_dirfd, new_path, flags)`—Renames a"] # [doc = " file or directory."] # [doc = ""] # [doc = " `renameat_with` is the same as [`renameat`] but adds an additional"] # [doc = " flags operand."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/renameat2.2.html"] # [cfg (any (apple , linux_kernel))] # [inline] # [doc (alias = "renameat2")] # [doc (alias = "renameatx_np")] pub fn renameat_with < P : path :: Arg , Q : path :: Arg , PFd : AsFd , QFd : AsFd > (old_dirfd : PFd , old_path : P , new_dirfd : QFd , new_path : Q , flags : RenameFlags ,) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | { backend :: fs :: syscalls :: renameat2 (old_dirfd . as_fd () , old_path , new_dirfd . as_fd () , new_path , flags ,) }) }) }
};
}
