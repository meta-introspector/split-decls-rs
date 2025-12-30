// Generated macro for impl_1450 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1450 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1450"}
// Dependencies: {}
# [unstable (feature = "dir_entry_ext2" , issue = "85573")] impl DirEntryExt2 for fs :: DirEntry { fn file_name_ref (& self) -> & OsStr { self . as_inner () . file_name_os_str () } }
};
}
