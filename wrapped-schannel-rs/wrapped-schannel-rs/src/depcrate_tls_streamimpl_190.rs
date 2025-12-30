// Generated macro for impl_190 (impl)
macro_rules! Depcrate_tls_streamimpl_190 {
() => {
// Module: crate::tls_stream
// Provides: {"impl_190"}
// Dependencies: {}
impl < S > MidHandshakeTlsStream < S > where S : Read + Write , { # [doc = " Restarts the handshake process."] pub fn handshake (mut self) -> Result < TlsStream < S > , HandshakeError < S > > { match self . inner . initialize () { Ok (_) => Ok (self . inner) , Err (ref e) if e . kind () == io :: ErrorKind :: WouldBlock => { Err (HandshakeError :: Interrupted (self)) } Err (e) => Err (HandshakeError :: Failure (e)) , } } }
};
}
