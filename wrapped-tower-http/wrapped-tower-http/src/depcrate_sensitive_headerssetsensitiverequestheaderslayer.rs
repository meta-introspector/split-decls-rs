// Generated macro for SetSensitiveRequestHeadersLayer (struct)
macro_rules! Depcrate_sensitive_headersSetSensitiveRequestHeadersLayer {
() => {
// Module: crate::sensitive_headers
// Provides: {"SetSensitiveRequestHeadersLayer"}
// Dependencies: {}
# [doc = " Mark request headers as [sensitive]."] # [doc = ""] # [doc = " Produces [`SetSensitiveRequestHeaders`] services."] # [doc = ""] # [doc = " See the [module docs](crate::sensitive_headers) for more details."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] # [derive (Clone , Debug)] pub struct SetSensitiveRequestHeadersLayer { headers : Arc < [HeaderName] > , }
};
}
