// Generated macro for impl_87 (impl)
macro_rules! Depcrate_common_test_streamimpl_87 {
() => {
// Module: crate::common::test_stream
// Provides: {"impl_87"}
// Dependencies: {}
impl AsyncRead for Good < '_ > { fn poll_read (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let mut buf2 = buf . initialize_unfilled () ; Poll :: Ready (match self . 0 . write_tls (buf2 . by_ref ()) { Ok (n) => { buf . advance (n) ; Ok (()) } Err (err) => Err (err) , }) } }
};
}
