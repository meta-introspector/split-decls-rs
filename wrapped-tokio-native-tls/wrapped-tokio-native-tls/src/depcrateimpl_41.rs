// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < S : AsyncRead + AsyncWrite + Unpin > Future for MidHandshake < S > { type Output = Result < TlsStream < S > , Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut_self = self . get_mut () ; let mut s = mut_self . 0 . take () . expect ("future polled after completion") ; s . get_mut () . context = cx as * mut _ as * mut () ; match s . handshake () { Ok (mut s) => { s . get_mut () . context = null_mut () ; Poll :: Ready (Ok (TlsStream (s))) } Err (HandshakeError :: WouldBlock (mut s)) => { s . get_mut () . context = null_mut () ; mut_self . 0 = Some (s) ; Poll :: Pending } Err (HandshakeError :: Failure (e)) => Poll :: Ready (Err (e)) , } } }
};
}
