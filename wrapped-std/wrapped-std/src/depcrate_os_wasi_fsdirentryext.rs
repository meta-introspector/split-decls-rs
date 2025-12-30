// Generated macro for DirEntryExt (trait)
macro_rules! Depcrate_os_wasi_fsDirEntryExt {
() => {
// Module: crate::os::wasi::fs
// Provides: {"DirEntryExt"}
// Dependencies: {}
# [doc = " WASI-specific extension methods for [`fs::DirEntry`]."] pub trait DirEntryExt { # [doc = " Returns the underlying `d_ino` field of the `dirent_t`"] fn ino (& self) -> u64 ; }
};
}
