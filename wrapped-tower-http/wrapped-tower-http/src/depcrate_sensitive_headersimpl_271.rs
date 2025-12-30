// Generated macro for impl_271 (impl)
macro_rules! Depcrate_sensitive_headersimpl_271 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_271"}
// Dependencies: {}
impl < S > SetSensitiveRequestHeaders < S > { # [doc = " Create a new [`SetSensitiveRequestHeaders`]."] pub fn new < I > (inner : S , headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { let headers = headers . into_iter () . collect :: < Vec < _ > > () ; Self :: from_shared (inner , headers . into ()) } # [doc = " Create a new [`SetSensitiveRequestHeaders`] from a shared slice of headers."] pub fn from_shared (inner : S , headers : Arc < [HeaderName] >) -> Self { Self { inner , headers } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `SetSensitiveRequestHeaders` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer < I > (headers : I) -> SetSensitiveRequestHeadersLayer where I : IntoIterator < Item = HeaderName > , { SetSensitiveRequestHeadersLayer :: new (headers) } }
};
}
