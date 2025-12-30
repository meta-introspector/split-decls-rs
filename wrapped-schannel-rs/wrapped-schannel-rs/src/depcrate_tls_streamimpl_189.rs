// Generated macro for impl_189 (impl)
macro_rules! Depcrate_tls_streamimpl_189 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_189"}
// Dependencies: {}
impl < S > MidHandshakeTlsStream < S > { # [doc = " Returns a shared reference to the inner stream."] pub fn get_ref (& self) -> & S { self . inner . get_ref () } # [doc = " Returns a mutable reference to the inner stream."] pub fn get_mut (& mut self) -> & mut S { self . inner . get_mut () } }
};
}
