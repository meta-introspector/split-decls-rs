// Generated macro for impl_2849 (impl)
macro_rules! Depcrate_pathimpl_2849 {
() => {
// Module: crate::path
// Provides: {"impl_2849"}
// Dependencies: {}
# [stable (feature = "path_buf_from_box" , since = "1.18.0")] impl From < Box < Path > > for PathBuf { # [doc = " Converts a <code>[Box]&lt;[Path]&gt;</code> into a [`PathBuf`]."] # [doc = ""] # [doc = " This conversion does not allocate or copy memory."] # [inline] fn from (boxed : Box < Path >) -> PathBuf { boxed . into_path_buf () } }
};
}
