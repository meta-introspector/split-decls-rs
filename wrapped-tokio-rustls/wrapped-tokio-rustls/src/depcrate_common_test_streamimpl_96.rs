// Generated macro for impl_96 (impl)
macro_rules! Depcrate_common_test_streamimpl_96 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_96"}
// Dependencies: {}
impl AsyncRead for Eof { fn poll_read (self : Pin < & mut Self > , _cx : & mut Context < '_ > , _buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
