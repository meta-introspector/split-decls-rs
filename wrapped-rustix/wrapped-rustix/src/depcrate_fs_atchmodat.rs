// Generated macro for chmodat (function)
macro_rules! Depcrate_fs_atchmodat {
() => {
// Module: crate::fs::at
// Provides: {"chmodat"}
// Dependencies: {}
# [doc = " `fchmodat(dirfd, path, mode, flags)`—Sets file or directory permissions."] # [doc = ""] # [doc = " Platform support for flags varies widely, for example on Linux"] # [doc = " [`AtFlags::SYMLINK_NOFOLLOW`] is not implemented and therefore"] # [doc = " [`io::Errno::OPNOTSUPP`] will be returned."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fchmodat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fchmodat.2.html"] # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] # [inline] # [doc (alias = "fchmodat")] pub fn chmodat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , mode : Mode , flags : AtFlags ,) -> io :: Result < () > { path . into_with_c_str (| path | backend :: fs :: syscalls :: chmodat (dirfd . as_fd () , path , mode , flags)) }
};
}
