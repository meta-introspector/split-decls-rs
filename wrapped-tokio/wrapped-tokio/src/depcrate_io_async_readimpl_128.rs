// Generated macro for impl_128 (impl)
macro_rules! Depcrate_io_async_readimpl_128 {
() => {
// Module: crate::io::async_read
// Provides: {"impl_128"}
// Dependencies: {}
impl AsyncRead for & [u8] { fn poll_read (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let amt = std :: cmp :: min (self . len () , buf . remaining ()) ; let (a , b) = self . split_at (amt) ; buf . put_slice (a) ; * self = b ; Poll :: Ready (Ok (())) } }
};
}
