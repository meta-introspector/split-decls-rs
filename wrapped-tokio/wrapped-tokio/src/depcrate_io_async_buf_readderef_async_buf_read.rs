// Generated macro for deref_async_buf_read (macro)
macro_rules! Depcrate_io_async_buf_readderef_async_buf_read {
() => {
// Module: crate::io::async_buf_read
// Provides: {"deref_async_buf_read"}
// Dependencies: {}
macro_rules ! deref_async_buf_read { () => { fn poll_fill_buf (self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result <& [u8] >> { Pin :: new (& mut ** self . get_mut ()) . poll_fill_buf (cx) } fn consume (mut self : Pin <& mut Self >, amt : usize) { Pin :: new (& mut ** self) . consume (amt) } } ; }
};
}
