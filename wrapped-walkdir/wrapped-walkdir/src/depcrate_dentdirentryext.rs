// Generated macro for DirEntryExt (trait)
macro_rules! Depcrate_dentDirEntryExt {
() => {
// Module: crate::dent
// Provides: {"DirEntryExt"}
// Dependencies: {}
# [doc = " Unix-specific extension methods for `walkdir::DirEntry`"] # [cfg (unix)] pub trait DirEntryExt { # [doc = " Returns the underlying `d_ino` field in the contained `dirent`"] # [doc = " structure."] fn ino (& self) -> u64 ; }
};
}
