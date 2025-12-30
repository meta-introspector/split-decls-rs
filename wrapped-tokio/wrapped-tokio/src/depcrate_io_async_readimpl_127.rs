// Generated macro for impl_127 (impl)
macro_rules! Depcrate_io_async_readimpl_127 {
() => {
// Module: crate::io::async_read
// Provides: {"impl_127"}
// Dependencies: {}
impl < P > AsyncRead for Pin < P > where P : DerefMut , P :: Target : AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { crate :: util :: pin_as_deref_mut (self) . poll_read (cx , buf) } }
};
}
