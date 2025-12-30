// Generated macro for impl_94 (impl)
macro_rules! Depcrate_common_test_streamimpl_94 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_94"}
// Dependencies: {}
impl AsyncWrite for Expected { fn poll_write (self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (buf . len ())) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
