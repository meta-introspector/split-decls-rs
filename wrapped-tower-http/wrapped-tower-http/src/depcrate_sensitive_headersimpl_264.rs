// Generated macro for impl_264 (impl)
macro_rules! Depcrate_sensitive_headersimpl_264 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_264"}
// Dependencies: {}
impl SetSensitiveHeadersLayer { # [doc = " Create a new [`SetSensitiveHeadersLayer`]."] pub fn new < I > (headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { let headers = headers . into_iter () . collect :: < Vec < _ > > () ; Self :: from_shared (headers . into ()) } # [doc = " Create a new [`SetSensitiveHeadersLayer`] from a shared slice of headers."] pub fn from_shared (headers : Arc < [HeaderName] >) -> Self { Self { headers } } }
};
}
