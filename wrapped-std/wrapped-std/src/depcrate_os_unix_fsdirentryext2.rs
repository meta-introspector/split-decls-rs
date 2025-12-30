// Generated macro for DirEntryExt2 (trait)
macro_rules! Depcrate_os_unix_fsDirEntryExt2 {
() => {
// Module: crate::os::unix::fs
// Provides: {"DirEntryExt2"}
// Dependencies: {}
# [doc = " Sealed Unix-specific extension methods for [`fs::DirEntry`]."] # [unstable (feature = "dir_entry_ext2" , issue = "85573")] pub trait DirEntryExt2 : Sealed { # [doc = " Returns a reference to the underlying `OsStr` of this entry's filename."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(dir_entry_ext2)]"] # [doc = " use std::os::unix::fs::DirEntryExt2;"] # [doc = " use std::{fs, io};"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let mut entries = fs::read_dir(\".\")?.collect::<Result<Vec<_>, io::Error>>()?;"] # [doc = "     entries.sort_unstable_by(|a, b| a.file_name_ref().cmp(b.file_name_ref()));"] # [doc = ""] # [doc = "     for p in entries {"] # [doc = "         println!(\"{p:?}\");"] # [doc = "     }"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] fn file_name_ref (& self) -> & OsStr ; }
};
}
