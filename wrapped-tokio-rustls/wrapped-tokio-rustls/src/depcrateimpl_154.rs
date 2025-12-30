// Generated macro for impl_154 (impl)
macro_rules! Depcrateimpl_154 {
() => {
// Module: crate
// Provides: {"impl_154"}
// Dependencies: {}
impl < T > AsyncBufRead for TlsStream < T > where T : AsyncRead + AsyncWrite + Unpin , { # [inline] fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . poll_fill_buf (cx) , Self :: Server (x) => Pin :: new (x) . poll_fill_buf (cx) , } } # [inline] fn consume (self : Pin < & mut Self > , amt : usize) { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . consume (amt) , Self :: Server (x) => Pin :: new (x) . consume (amt) , } } }
};
}
