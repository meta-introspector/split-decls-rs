// Generated macro for impl_2854 (impl)
macro_rules! Depcrate_pathimpl_2854 {
() => {
// Module: crate::path
// Provides: {"impl_2854"}
// Dependencies: {}
# [stable (feature = "from_path_buf_for_os_string" , since = "1.14.0")] impl From < PathBuf > for OsString { # [doc = " Converts a [`PathBuf`] into an [`OsString`]"] # [doc = ""] # [doc = " This conversion does not allocate or copy memory."] # [inline] fn from (path_buf : PathBuf) -> OsString { path_buf . inner } }
};
}
