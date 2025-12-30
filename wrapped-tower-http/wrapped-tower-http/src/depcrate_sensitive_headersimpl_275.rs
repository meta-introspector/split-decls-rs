// Generated macro for impl_275 (impl)
macro_rules! Depcrate_sensitive_headersimpl_275 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_275"}
// Dependencies: {}
impl < S > Layer < S > for SetSensitiveResponseHeadersLayer { type Service = SetSensitiveResponseHeaders < S > ; fn layer (& self , inner : S) -> Self :: Service { SetSensitiveResponseHeaders { inner , headers : self . headers . clone () , } } }
};
}
