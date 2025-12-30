// Generated macro for impl_140 (impl)
macro_rules! Depcrate_io_async_seekimpl_140 {
() => {
// Module: crate::io::async_seek
// Provides: {"impl_140"}
// Dependencies: {}
impl < P > AsyncSeek for Pin < P > where P : DerefMut , P :: Target : AsyncSeek , { fn start_seek (self : Pin < & mut Self > , pos : SeekFrom) -> io :: Result < () > { crate :: util :: pin_as_deref_mut (self) . start_seek (pos) } fn poll_complete (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < u64 > > { crate :: util :: pin_as_deref_mut (self) . poll_complete (cx) } }
};
}
