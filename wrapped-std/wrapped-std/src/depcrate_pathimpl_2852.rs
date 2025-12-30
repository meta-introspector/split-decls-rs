// Generated macro for impl_2852 (impl)
macro_rules! Depcrate_pathimpl_2852 {
() => {
// Module: crate::path
// Provides: {"impl_2852"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + AsRef < OsStr > > From < & T > for PathBuf { # [doc = " Converts a borrowed [`OsStr`] to a [`PathBuf`]."] # [doc = ""] # [doc = " Allocates a [`PathBuf`] and copies the data into it."] # [inline] fn from (s : & T) -> PathBuf { PathBuf :: from (s . as_ref () . to_os_string ()) } }
};
}
