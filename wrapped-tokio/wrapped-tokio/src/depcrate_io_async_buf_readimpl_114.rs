// Generated macro for impl_114 (impl)
macro_rules! Depcrate_io_async_buf_readimpl_114 {
() => {
// Module: crate::io::async_buf_read
// Provides: {"impl_114"}
// Dependencies: {}
impl AsyncBufRead for & [u8] { fn poll_fill_buf (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (Ok (* self)) } fn consume (mut self : Pin < & mut Self > , amt : usize) { * self = & self [amt ..] ; } }
};
}
