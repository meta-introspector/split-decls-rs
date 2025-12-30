// Generated macro for impl_2853 (impl)
macro_rules! Depcrate_pathimpl_2853 {
() => {
// Module: crate::path
// Provides: {"impl_2853"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl From < OsString > for PathBuf { # [doc = " Converts an [`OsString`] into a [`PathBuf`]."] # [doc = ""] # [doc = " This conversion does not allocate or copy memory."] # [inline] fn from (s : OsString) -> PathBuf { PathBuf { inner : s } } }
};
}
