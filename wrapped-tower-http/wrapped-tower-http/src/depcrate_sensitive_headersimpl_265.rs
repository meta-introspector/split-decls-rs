// Generated macro for impl_265 (impl)
macro_rules! Depcrate_sensitive_headersimpl_265 {
() => {
// Module: crate::sensitive_headers
// Provides: {"impl_265"}
// Dependencies: {}
impl < S > Layer < S > for SetSensitiveHeadersLayer { type Service = SetSensitiveHeaders < S > ; fn layer (& self , inner : S) -> Self :: Service { SetSensitiveRequestHeaders :: from_shared (SetSensitiveResponseHeaders :: from_shared (inner , self . headers . clone ()) , self . headers . clone () ,) } }
};
}
