// Generated macro for impl_881 (impl)
macro_rules! Depcrate_tlsimpl_881 {
() => {
// Module: crate::tls
// Provides: {"impl_881"}
// Dependencies: {}
impl AsyncWrite for TlsStream { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { let pin = self . get_mut () ; match pin . state { State :: Handshaking (ref mut accept) => match ready ! (Pin :: new (accept) . poll (cx)) { Ok (mut stream) => { let result = Pin :: new (& mut stream) . poll_write (cx , buf) ; pin . state = State :: Streaming (stream) ; result } Err (err) => Poll :: Ready (Err (err)) , } , State :: Streaming (ref mut stream) => Pin :: new (stream) . poll_write (cx , buf) , } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { match self . state { State :: Handshaking (_) => Poll :: Ready (Ok (())) , State :: Streaming (ref mut stream) => Pin :: new (stream) . poll_flush (cx) , } } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { match self . state { State :: Handshaking (_) => Poll :: Ready (Ok (())) , State :: Streaming (ref mut stream) => Pin :: new (stream) . poll_shutdown (cx) , } } }
};
}
