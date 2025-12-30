// Generated macro for impl_137 (impl)
macro_rules! Depcrate_serverimpl_137 {
() => {
// Module: crate::server
// Provides: {"impl_137"}
// Dependencies: {}
impl < IO : AsyncRead + AsyncWrite + Unpin > Future for FallibleAccept < IO > { type Output = Result < TlsStream < IO > , (io :: Error , IO) > ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}
