// Generated macro for impl_2861 (impl)
macro_rules! Depcrate_pathimpl_2861 {
() => {
// Module: crate::path
// Provides: {"impl_2861"}
// Dependencies: {}
# [stable (feature = "path_buf_deref_mut" , since = "1.68.0")] impl ops :: DerefMut for PathBuf { # [inline] fn deref_mut (& mut self) -> & mut Path { Path :: from_inner_mut (& mut self . inner) } }
};
}
