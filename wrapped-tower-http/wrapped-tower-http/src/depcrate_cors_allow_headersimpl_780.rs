// Generated macro for impl_780 (impl)
macro_rules! Depcrate_cors_allow_headersimpl_780 {
() => {
// Module: crate::cors::allow_headers
// Provides: {"impl_780"}
// Dependencies: {}
impl < const N : usize > From < [HeaderName ; N] > for AllowHeaders { fn from (arr : [HeaderName ; N]) -> Self { # [allow (deprecated)] Self :: list (array :: IntoIter :: new (arr)) } }
};
}
