// Generated macro for impl_91 (impl)
macro_rules! Depcrate_common_test_streamimpl_91 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_91"}
// Dependencies: {}
impl AsyncWrite for Pending { fn poll_write (self : Pin < & mut Self > , _cx : & mut Context < '_ > , _buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Pending } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
