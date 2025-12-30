// Generated macro for impl_186 (impl)
macro_rules! Depcrate_tls_streamimpl_186 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_186"}
// Dependencies: {}
impl < S > fmt :: Debug for TlsStream < S > where S : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("TlsStream") . field ("stream" , & self . stream) . finish () } }
};
}
