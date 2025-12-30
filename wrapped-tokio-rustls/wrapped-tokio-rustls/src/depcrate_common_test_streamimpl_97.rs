// Generated macro for impl_97 (impl)
macro_rules! Depcrate_common_test_streamimpl_97 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_97"}
// Dependencies: {}
impl AsyncWrite for Eof { fn poll_write (self : Pin < & mut Self > , _cx : & mut Context < '_ > , _buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (0)) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
};
}
