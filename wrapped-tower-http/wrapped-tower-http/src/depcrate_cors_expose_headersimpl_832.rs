// Generated macro for impl_832 (impl)
macro_rules! Depcrate_cors_expose_headersimpl_832 {
() => {
// Module: crate::cors::expose_headers
// Provides: {"impl_832"}
// Dependencies: {}
impl < const N : usize > From < [HeaderName ; N] > for ExposeHeaders { fn from (arr : [HeaderName ; N]) -> Self { # [allow (deprecated)] Self :: list (array :: IntoIter :: new (arr)) } }
};
}
