// Generated macro for SetSensitiveHeadersLayer (struct)
macro_rules! Depcrate_sensitive_headersSetSensitiveHeadersLayer {
() => {
// Module: crate::sensitive_headers
// Provides: {"SetSensitiveHeadersLayer"}
// Dependencies: {}
# [doc = " Mark headers as [sensitive] on both requests and responses."] # [doc = ""] # [doc = " Produces [`SetSensitiveHeaders`] services."] # [doc = ""] # [doc = " See the [module docs](crate::sensitive_headers) for more details."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] # [derive (Clone , Debug)] pub struct SetSensitiveHeadersLayer { headers : Arc < [HeaderName] > , }
};
}
