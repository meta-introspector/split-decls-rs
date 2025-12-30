// Generated macro for SetSensitiveHeaders (type)
macro_rules! Depcrate_sensitive_headersSetSensitiveHeaders {
() => {
// Module: crate::sensitive_headers
// Provides: {"SetSensitiveHeaders"}
// Dependencies: {}
# [doc = " Mark headers as [sensitive] on both requests and responses."] # [doc = ""] # [doc = " See the [module docs](crate::sensitive_headers) for more details."] # [doc = ""] # [doc = " [sensitive]: https://docs.rs/http/latest/http/header/struct.HeaderValue.html#method.set_sensitive"] pub type SetSensitiveHeaders < S > = SetSensitiveRequestHeaders < SetSensitiveResponseHeaders < S > > ;
};
}
