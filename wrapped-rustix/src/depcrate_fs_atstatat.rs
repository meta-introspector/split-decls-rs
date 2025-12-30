// Generated macro for statat (function)
macro_rules! Depcrate_fs_atstatat {
() => {
// Module: crate::fs::at
// Provides: {"statat"}
// Dependencies: {}
# [doc = " `fstatat(dirfd, path, flags)`—Queries metadata for a file or directory."] # [doc = ""] # [doc = " [`Mode::from_raw_mode`] and [`FileType::from_raw_mode`] may be used to"] # [doc = " interpret the `st_mode` field."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/fstatat.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/fstatat.2.html"] # [doc = " [`Mode::from_raw_mode`]: crate::fs::Mode::from_raw_mode"] # [doc = " [`FileType::from_raw_mode`]: crate::fs::FileType::from_raw_mode"] # [cfg (not (target_os = "espidf"))] # [inline] # [doc (alias = "fstatat")] pub fn statat < P : path :: Arg , Fd : AsFd > (dirfd : Fd , path : P , flags : AtFlags) -> io :: Result < Stat > { path . into_with_c_str (| path | backend :: fs :: syscalls :: statat (dirfd . as_fd () , path , flags)) }
};
}
