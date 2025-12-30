// Generated macro for FileTypeExt (trait)
macro_rules! Depcrate_os_windows_fsFileTypeExt {
() => {
// Module: crate::os::windows::fs
// Provides: {"FileTypeExt"}
// Dependencies: {}
# [doc = " Windows-specific extensions to [`fs::FileType`]."] # [doc = ""] # [doc = " On Windows, a symbolic link knows whether it is a file or directory."] # [stable (feature = "windows_file_type_ext" , since = "1.64.0")] pub trait FileTypeExt : Sealed { # [doc = " Returns `true` if this file type is a symbolic link that is also a directory."] # [stable (feature = "windows_file_type_ext" , since = "1.64.0")] fn is_symlink_dir (& self) -> bool ; # [doc = " Returns `true` if this file type is a symbolic link that is also a file."] # [stable (feature = "windows_file_type_ext" , since = "1.64.0")] fn is_symlink_file (& self) -> bool ; }
};
}
