// Generated macro for impl_2855 (impl)
macro_rules! Depcrate_pathimpl_2855 {
() => {
// Module: crate::path
// Provides: {"impl_2855"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl From < String > for PathBuf { # [doc = " Converts a [`String`] into a [`PathBuf`]"] # [doc = ""] # [doc = " This conversion does not allocate or copy memory."] # [inline] fn from (s : String) -> PathBuf { PathBuf :: from (OsString :: from (s)) } }
};
}
