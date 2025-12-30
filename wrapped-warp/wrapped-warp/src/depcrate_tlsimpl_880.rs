// Generated macro for impl_880 (impl)
macro_rules! Depcrate_tlsimpl_880 {
() => {
// Module: crate::tls
// Provides: {"impl_880"}
// Dependencies: {}
impl AsyncRead for TlsStream { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let pin = self . get_mut () ; match pin . state { State :: Handshaking (ref mut accept) => match ready ! (Pin :: new (accept) . poll (cx)) { Ok (mut stream) => { let result = Pin :: new (& mut stream) . poll_read (cx , buf) ; pin . state = State :: Streaming (stream) ; result } Err (err) => Poll :: Ready (Err (err)) , } , State :: Streaming (ref mut stream) => Pin :: new (stream) . poll_read (cx , buf) , } } }
};
}
