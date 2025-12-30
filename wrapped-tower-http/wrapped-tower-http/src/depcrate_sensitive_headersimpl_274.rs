// Generated macro for impl_274 (impl)
macro_rules! Depcrate_sensitive_headersimpl_274 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_274"}
// Dependencies: {}
impl SetSensitiveResponseHeadersLayer { # [doc = " Create a new [`SetSensitiveResponseHeadersLayer`]."] pub fn new < I > (headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { let headers = headers . into_iter () . collect :: < Vec < _ > > () ; Self :: from_shared (headers . into ()) } # [doc = " Create a new [`SetSensitiveResponseHeadersLayer`] from a shared slice of headers."] pub fn from_shared (headers : Arc < [HeaderName] >) -> Self { Self { headers } } }
};
}
