// Generated macro for impl_1264 (impl)
macro_rules! Depcrate_validate_requestimpl_1264 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1264"}
// Dependencies: {}
impl < ResBody > ValidateRequestHeaderLayer < AcceptHeader < ResBody > > { # [doc = " Validate requests have the required Accept header."] # [doc = ""] # [doc = " The `Accept` header is required to be `*/*`, `type/*` or `type/subtype`,"] # [doc = " as configured."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `header_value` is not in the form: `type/subtype`, such as `application/json`"] # [doc = " See `AcceptHeader::new` for when this method panics."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use http_body_util::Full;"] # [doc = " use bytes::Bytes;"] # [doc = " use tower_http::validate_request::{AcceptHeader, ValidateRequestHeaderLayer};"] # [doc = ""] # [doc = " let layer = ValidateRequestHeaderLayer::<AcceptHeader<Full<Bytes>>>::accept(\"application/json\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`Accept`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept"] pub fn accept (value : & str) -> Self where ResBody : Default , { Self :: custom (AcceptHeader :: new (value)) } }
};
}
