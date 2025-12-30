// Generated macro for impl_1819 (impl)
macro_rules! Depcrate_os_windows_fsimpl_1819 {
() => {
// Module: crate::os::windows::fs
// Provides: {"impl_1819"}
// Dependencies: {}
# [stable (feature = "windows_file_type_ext" , since = "1.64.0")] impl FileTypeExt for fs :: FileType { fn is_symlink_dir (& self) -> bool { self . as_inner () . is_symlink_dir () } fn is_symlink_file (& self) -> bool { self . as_inner () . is_symlink_file () } }
};
}
