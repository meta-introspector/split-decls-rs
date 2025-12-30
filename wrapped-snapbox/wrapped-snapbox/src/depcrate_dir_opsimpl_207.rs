// Generated macro for impl_207 (impl)
macro_rules! Depcrate_dir_opsimpl_207 {
() => {
// Module: crate::dir::ops
// Provides: {"impl_207"}
// Dependencies: {}
# [cfg (feature = "dir")] impl Walk { pub fn new (path : & std :: path :: Path) -> Self { Self { inner : walkdir :: WalkDir :: new (path) . into_iter () , } } }
};
}
