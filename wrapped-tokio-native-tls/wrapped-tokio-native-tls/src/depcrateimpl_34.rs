// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < F , S > Future for StartedHandshakeFuture < F , S > where F : FnOnce (AllowStd < S > ,) -> Result < native_tls :: TlsStream < AllowStd < S > > , HandshakeError < AllowStd < S > > > + Unpin , S : Unpin , AllowStd < S > : Read + Write , { type Output = Result < StartedHandshake < S > , Error > ; fn poll (mut self : Pin < & mut Self > , ctx : & mut Context < '_ > ,) -> Poll < Result < StartedHandshake < S > , Error > > { let inner = self . 0 . take () . expect ("future polled after completion") ; let stream = AllowStd { inner : inner . stream , context : ctx as * mut _ as * mut () , } ; match (inner . f) (stream) { Ok (mut s) => { s . get_mut () . context = null_mut () ; Poll :: Ready (Ok (StartedHandshake :: Done (TlsStream (s)))) } Err (HandshakeError :: WouldBlock (mut s)) => { s . get_mut () . context = null_mut () ; Poll :: Ready (Ok (StartedHandshake :: Mid (s))) } Err (HandshakeError :: Failure (e)) => Poll :: Ready (Err (e)) , } } }
};
}
