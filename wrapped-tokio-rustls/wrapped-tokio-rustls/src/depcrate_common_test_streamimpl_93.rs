// Generated macro for impl_93 (impl)
macro_rules! Depcrate_common_test_streamimpl_93 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_93"}
// Dependencies: {}
impl AsyncRead for Expected { fn poll_read (self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let this = self . get_mut () ; let n = std :: io :: Read :: read (& mut this . 0 , buf . initialize_unfilled ()) ? ; buf . advance (n) ; Poll :: Ready (Ok (())) } }
};
}
