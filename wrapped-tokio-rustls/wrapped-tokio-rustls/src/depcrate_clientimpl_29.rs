// Generated macro for impl_29 (impl)
macro_rules! Depcrate_clientimpl_29 {
() => {
// Module: crate::client
// Provides: {"impl_29"}
// Dependencies: {}
impl < IO : AsyncRead + AsyncWrite + Unpin > Future for FallibleConnect < IO > { type Output = Result < TlsStream < IO > , (io :: Error , IO) > ; # [inline] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}
