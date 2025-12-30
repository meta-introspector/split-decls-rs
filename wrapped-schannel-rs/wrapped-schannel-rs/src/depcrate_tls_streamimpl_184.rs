// Generated macro for impl_184 (impl)
macro_rules! Depcrate_tls_streamimpl_184 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_184"}
// Dependencies: {}
impl < S : fmt :: Debug + Any > fmt :: Display for HandshakeError < S > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let desc = match * self { HandshakeError :: Failure (_) => "failed to perform handshake" , HandshakeError :: Interrupted (_) => "interrupted performing handshake" , } ; write ! (f , "{}" , desc) ? ; if let Some (e) = self . source () { write ! (f , ": {}" , e) ? ; } Ok (()) } }
};
}
