// Generated macro for impl_90 (impl)
macro_rules! Depcrate_common_test_streamimpl_90 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_90"}
// Dependencies: {}
impl AsyncRead for Pending { fn poll_read (self : Pin < & mut Self > , _cx : & mut Context < '_ > , _ : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Poll :: Pending } }
};
}
