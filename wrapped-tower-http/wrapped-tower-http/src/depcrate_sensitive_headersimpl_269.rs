// Generated macro for impl_269 (impl)
macro_rules! Depcrate_sensitive_headersimpl_269 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_269"}
// Dependencies: {}
impl < S > Layer < S > for SetSensitiveRequestHeadersLayer { type Service = SetSensitiveRequestHeaders < S > ; fn layer (& self , inner : S) -> Self :: Service { SetSensitiveRequestHeaders { inner , headers : self . headers . clone () , } } }
};
}
