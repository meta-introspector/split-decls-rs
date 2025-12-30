// Generated macro for impl_187 (impl)
macro_rules! Depcrate_tls_streamimpl_187 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_187"}
// Dependencies: {}
impl < S > TlsStream < S > { # [doc = " Returns a reference to the wrapped stream."] pub fn get_ref (& self) -> & S { & self . stream } # [doc = " Returns a mutable reference to the wrapped stream."] pub fn get_mut (& mut self) -> & mut S { & mut self . stream } # [doc = " Indicates if this stream is the server- or client-side of a TLS session."] pub fn is_server (& self) -> bool { self . server } }
};
}
