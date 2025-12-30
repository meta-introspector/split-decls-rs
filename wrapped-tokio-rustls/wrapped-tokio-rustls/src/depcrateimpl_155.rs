// Generated macro for impl_155 (impl)
macro_rules! Depcrateimpl_155 {
() => {
// Module: crate
// Provides: {"impl_155"}
// Dependencies: {}
impl < T > AsyncWrite for TlsStream < T > where T : AsyncRead + AsyncWrite + Unpin , { # [inline] fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . poll_write (cx , buf) , Self :: Server (x) => Pin :: new (x) . poll_write (cx , buf) , } } # [inline] fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . poll_write_vectored (cx , bufs) , Self :: Server (x) => Pin :: new (x) . poll_write_vectored (cx , bufs) , } } # [inline] fn is_write_vectored (& self) -> bool { match self { Self :: Client (x) => x . is_write_vectored () , Self :: Server (x) => x . is_write_vectored () , } } # [inline] fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . poll_flush (cx) , Self :: Server (x) => Pin :: new (x) . poll_flush (cx) , } } # [inline] fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . poll_shutdown (cx) , Self :: Server (x) => Pin :: new (x) . poll_shutdown (cx) , } } }
};
}
