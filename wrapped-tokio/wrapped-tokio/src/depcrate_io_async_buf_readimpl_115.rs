// Generated macro for impl_115 (impl)
macro_rules! Depcrate_io_async_buf_readimpl_115 {
() => {
// Module: crate::io::async_buf_read
// Provides: {"impl_115"}
// Dependencies: {}
impl < T : AsRef < [u8] > + Unpin > AsyncBufRead for io :: Cursor < T > { fn poll_fill_buf (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (io :: BufRead :: fill_buf (self . get_mut ())) } fn consume (self : Pin < & mut Self > , amt : usize) { io :: BufRead :: consume (self . get_mut () , amt) ; } }
};
}
