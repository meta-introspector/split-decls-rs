// Generated macro for impl_153 (impl)
macro_rules! Depcrateimpl_153 {
() => {
// Module: crate
// Provides: {"impl_153"}
// Dependencies: {}
impl < T > AsyncRead for TlsStream < T > where T : AsyncRead + AsyncWrite + Unpin , { # [inline] fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { match self . get_mut () { Self :: Client (x) => Pin :: new (x) . poll_read (cx , buf) , Self :: Server (x) => Pin :: new (x) . poll_read (cx , buf) , } } }
};
}
