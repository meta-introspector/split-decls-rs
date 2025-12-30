// Generated macro for DirEntryExt (trait)
macro_rules! Depcrate_os_unix_fsDirEntryExt {
() => {
// Module: crate::os::unix::fs
// Provides: {"DirEntryExt"}
// Dependencies: {}
# [doc = " Unix-specific extension methods for [`fs::DirEntry`]."] # [stable (feature = "dir_entry_ext" , since = "1.1.0")] pub trait DirEntryExt { # [doc = " Returns the underlying `d_ino` field in the contained `dirent`"] # [doc = " structure."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::fs;"] # [doc = " use std::os::unix::fs::DirEntryExt;"] # [doc = ""] # [doc = " if let Ok(entries) = fs::read_dir(\".\") {"] # [doc = "     for entry in entries {"] # [doc = "         if let Ok(entry) = entry {"] # [doc = "             // Here, `entry` is a `DirEntry`."] # [doc = "             println!(\"{:?}: {}\", entry.file_name(), entry.ino());"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "dir_entry_ext" , since = "1.1.0")] fn ino (& self) -> u64 ; }
};
}
