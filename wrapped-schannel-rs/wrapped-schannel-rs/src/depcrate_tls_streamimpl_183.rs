// Generated macro for impl_183 (impl)
macro_rules! Depcrate_tls_streamimpl_183 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_183"}
// Dependencies: {}
impl < S : fmt :: Debug + Any > Error for HandshakeError < S > { fn source (& self) -> Option < & (dyn Error + 'static) > { match * self { HandshakeError :: Failure (ref e) => Some (e) , HandshakeError :: Interrupted (_) => None , } } }
};
}
