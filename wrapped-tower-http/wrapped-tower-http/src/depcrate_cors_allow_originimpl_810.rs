// Generated macro for impl_810 (impl)
macro_rules! Depcrate_cors_allow_originimpl_810 {
() => {
// Module: crate::cors::allow_origin
// Provides: {"impl_810"}
// Dependencies: {}
impl < const N : usize > From < [HeaderValue ; N] > for AllowOrigin { fn from (arr : [HeaderValue ; N]) -> Self { # [allow (deprecated)] Self :: list (array :: IntoIter :: new (arr)) } }
};
}
