// Generated macro for impl_45 (impl)
macro_rules! Depcrate_fileimpl_45 {
() => {
// Module: crate::file
// Provides: {"impl_45"}
// Dependencies: {}
impl Type { # [doc = " Returns true if this type represents a character file, which is"] # [doc = " typically an LPT device or a console."] pub fn is_char (& self) -> bool { self . 0 == :: windows_sys :: Win32 :: Storage :: FileSystem :: FILE_TYPE_CHAR } # [doc = " Returns true if this type represents a disk file."] pub fn is_disk (& self) -> bool { self . 0 == :: windows_sys :: Win32 :: Storage :: FileSystem :: FILE_TYPE_DISK } # [doc = " Returns true if this type represents a sock, named pipe or an"] # [doc = " anonymous pipe."] pub fn is_pipe (& self) -> bool { self . 0 == :: windows_sys :: Win32 :: Storage :: FileSystem :: FILE_TYPE_PIPE } # [doc = " Returns true if this type is not known."] # [doc = ""] # [doc = " Note that this never corresponds to a failure."] pub fn is_unknown (& self) -> bool { self . 0 == :: windows_sys :: Win32 :: Storage :: FileSystem :: FILE_TYPE_UNKNOWN } }
};
}
