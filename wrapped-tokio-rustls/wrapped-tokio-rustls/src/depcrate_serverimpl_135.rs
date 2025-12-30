// Generated macro for impl_135 (impl)
macro_rules! Depcrate_serverimpl_135 {
() => {
// Module: crate::server
// Provides: {"impl_135"}
// Dependencies: {}
impl < IO : AsyncRead + AsyncWrite + Unpin > Future for Accept < IO > { type Output = io :: Result < TlsStream < IO > > ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) . map_err (| (err , _) | err) } }
};
}
