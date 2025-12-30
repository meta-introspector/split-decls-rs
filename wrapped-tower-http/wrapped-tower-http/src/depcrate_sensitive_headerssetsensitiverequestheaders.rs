// Generated macro for SetSensitiveRequestHeaders (struct)
macro_rules! Depcrate_sensitive_headersSetSensitiveRequestHeaders {
() => {
// Module: crate::sensitive_headers
// Provides: {"SetSensitiveRequestHeaders"}
// Dependencies: {}
# [doc = " Mark request headers as [sensitive]."] # [doc = ""] # [doc = " See the [module docs](crate::sensitive_headers) for more details."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] # [derive (Clone , Debug)] pub struct SetSensitiveRequestHeaders < S > { inner : S , headers : Arc < [HeaderName] > , }
};
}
