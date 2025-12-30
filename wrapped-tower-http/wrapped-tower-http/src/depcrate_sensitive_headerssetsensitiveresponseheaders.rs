// Generated macro for SetSensitiveResponseHeaders (struct)
macro_rules! Depcrate_sensitive_headersSetSensitiveResponseHeaders {
() => {
// Module: crate::sensitive_headers
// Provides: {"SetSensitiveResponseHeaders"}
// Dependencies: {}
# [doc = " Mark response headers as [sensitive]."] # [doc = ""] # [doc = " See the [module docs](crate::sensitive_headers) for more details."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] # [derive (Clone , Debug)] pub struct SetSensitiveResponseHeaders < S > { inner : S , headers : Arc < [HeaderName] > , }
};
}
