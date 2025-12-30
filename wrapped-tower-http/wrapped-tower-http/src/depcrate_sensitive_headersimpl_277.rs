// Generated macro for impl_277 (impl)
macro_rules! Depcrate_sensitive_headersimpl_277 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_277"}
// Dependencies: {}
impl < S > SetSensitiveResponseHeaders < S > { # [doc = " Create a new [`SetSensitiveResponseHeaders`]."] pub fn new < I > (inner : S , headers : I) -> Self where I : IntoIterator < Item = HeaderName > , { let headers = headers . into_iter () . collect :: < Vec < _ > > () ; Self :: from_shared (inner , headers . into ()) } # [doc = " Create a new [`SetSensitiveResponseHeaders`] from a shared slice of headers."] pub fn from_shared (inner : S , headers : Arc < [HeaderName] >) -> Self { Self { inner , headers } } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `SetSensitiveResponseHeaders` middleware."] # [doc = ""] # [doc = " [`Layer`]: tower_layer::Layer"] pub fn layer < I > (headers : I) -> SetSensitiveResponseHeadersLayer where I : IntoIterator < Item = HeaderName > , { SetSensitiveResponseHeadersLayer :: new (headers) } }
};
}
