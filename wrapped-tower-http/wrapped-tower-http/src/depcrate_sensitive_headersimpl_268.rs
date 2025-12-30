// Generated macro for impl_268 (impl)
macro_rules! Depcrate_sensitive_headersimpl_268 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_268"}
// Dependencies: {}
impl SetSensitiveRequestHeadersLayer { # [doc = " Create a new [`SetSensitiveRequestHeadersLayer`]."] pub fn new < I > (headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { let headers = headers . into_iter () . collect :: < Vec < _ > > () ; Self :: from_shared (headers . into ()) } # [doc = " Create a new [`SetSensitiveRequestHeadersLayer`] from a shared slice of headers."] pub fn from_shared (headers : Arc < [HeaderName] >) -> Self { Self { headers } } }
};
}
