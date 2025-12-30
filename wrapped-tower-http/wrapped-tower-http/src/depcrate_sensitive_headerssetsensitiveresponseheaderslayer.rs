// Generated macro for SetSensitiveResponseHeadersLayer (struct)
macro_rules! Depcrate_sensitive_headersSetSensitiveResponseHeadersLayer {
() => {
// Module: crate::sensitive_headers
// Provides: {"SetSensitiveResponseHeadersLayer"}
// Dependencies: {}
# [doc = " Mark response headers as [sensitive]."] # [doc = ""] # [doc = " Produces [`SetSensitiveResponseHeaders`] services."] # [doc = ""] # [doc = " See the [module docs](crate::sensitive_headers) for more details."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] # [derive (Clone , Debug)] pub struct SetSensitiveResponseHeadersLayer { headers : Arc < [HeaderName] > , }
};
}
