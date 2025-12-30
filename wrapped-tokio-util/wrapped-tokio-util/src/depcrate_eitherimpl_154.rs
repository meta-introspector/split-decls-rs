// Generated macro for impl_154 (impl)
macro_rules! Depcrate_eitherimpl_154 {
() => {
// Module: crate::either
// Provides: {"impl_154"}
// Dependencies: {}
impl < L , R > AsyncSeek for Either < L , R > where L : AsyncSeek , R : AsyncSeek , { fn start_seek (self : Pin < & mut Self > , position : SeekFrom) -> Result < () > { delegate_call ! (self . start_seek (position)) } fn poll_complete (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < u64 > > { delegate_call ! (self . poll_complete (cx)) } }
};
}
