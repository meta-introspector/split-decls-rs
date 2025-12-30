// Generated macro for impl_141 (impl)
macro_rules! Depcrate_serverimpl_141 {
() => {
// Module: crate::server
// Provides: {"impl_141"}
// Dependencies: {}
impl < IO > AsyncRead for TlsStream < IO > where IO : AsyncRead + AsyncWrite + Unpin , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let data = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let len = data . len () . min (buf . remaining ()) ; buf . put_slice (& data [.. len]) ; self . consume (len) ; Poll :: Ready (Ok (())) } }
};
}
