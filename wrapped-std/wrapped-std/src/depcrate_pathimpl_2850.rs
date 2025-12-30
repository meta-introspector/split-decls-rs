// Generated macro for impl_2850 (impl)
macro_rules! Depcrate_pathimpl_2850 {
() => {
// Module: crate::path
// Provides: {"impl_2850"}
// Dependencies: {}
# [stable (feature = "box_from_path_buf" , since = "1.20.0")] impl From < PathBuf > for Box < Path > { # [doc = " Converts a [`PathBuf`] into a <code>[Box]&lt;[Path]&gt;</code>."] # [doc = ""] # [doc = " This conversion currently should not allocate memory,"] # [doc = " but this behavior is not guaranteed on all platforms or in all future versions."] # [inline] fn from (p : PathBuf) -> Box < Path > { p . into_boxed_path () } }
};
}
