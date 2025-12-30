// Generated macro for impl_28 (impl)
macro_rules! Depcrate_clientimpl_28 {
() => {
// Module: crate::client
// Provides: {"impl_28"}
// Dependencies: {}
impl < IO : AsyncRead + AsyncWrite + Unpin > Future for Connect < IO > { type Output = io :: Result < TlsStream < IO > > ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) . map_err (| (err , _) | err) } }
};
}
