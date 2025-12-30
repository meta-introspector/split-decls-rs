// Generated macro for impl_140 (impl)
macro_rules! Depcrate_sessionimpl_140 {
() => {
// Module: crate::session
// Provides: {"impl_140"}
// Dependencies: {}
impl ScpFileStat { # [doc = " Returns the size of the remote file."] pub fn size (& self) -> u64 { self . stat . st_size as u64 } # [doc = " Returns the listed mode of the remote file."] pub fn mode (& self) -> i32 { self . stat . st_mode as i32 } # [doc = " Returns whether the remote file is a directory."] pub fn is_dir (& self) -> bool { self . mode () & (libc :: S_IFMT as i32) == (libc :: S_IFDIR as i32) } # [doc = " Returns whether the remote file is a regular file."] pub fn is_file (& self) -> bool { self . mode () & (libc :: S_IFMT as i32) == (libc :: S_IFREG as i32) } }
};
}
