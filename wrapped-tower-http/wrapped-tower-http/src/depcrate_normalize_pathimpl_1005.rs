// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_normalize_pathimpl_1005 {
() => {
// Module: crate::normalize_path
// Provides: {"impl_1005"}
// Dependencies: {}
impl NormalizePathLayer { # [doc = " Create a new [`NormalizePathLayer`]."] # [doc = ""] # [doc = " Any trailing slashes from request paths will be removed. For example, a request with `/foo/`"] # [doc = " will be changed to `/foo` before reaching the inner service."] pub fn trim_trailing_slash () -> Self { NormalizePathLayer { mode : NormalizeMode :: Trim , } } # [doc = " Create a new [`NormalizePathLayer`]."] # [doc = ""] # [doc = " Request paths without trailing slash will be appended with a trailing slash. For example, a request with `/foo`"] # [doc = " will be changed to `/foo/` before reaching the inner service."] pub fn append_trailing_slash () -> Self { NormalizePathLayer { mode : NormalizeMode :: Append , } } }
};
}
