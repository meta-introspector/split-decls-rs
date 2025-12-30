// Generated macro for impl_152 (impl)
macro_rules! Depcrate_eitherimpl_152 {
() => {
// Module: crate::either
// Provides: {"impl_152"}
// Dependencies: {}
impl < L , R > AsyncRead for Either < L , R > where L : AsyncRead , R : AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < Result < () > > { delegate_call ! (self . poll_read (cx , buf)) } }
};
}
