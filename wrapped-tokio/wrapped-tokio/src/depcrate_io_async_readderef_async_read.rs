// Generated macro for deref_async_read (macro)
macro_rules! Depcrate_io_async_readderef_async_read {
() => {
// Module: crate::io::async_read
// Provides: {"deref_async_read"}
// Dependencies: {}
macro_rules ! deref_async_read { () => { fn poll_read (mut self : Pin <& mut Self >, cx : & mut Context <'_ >, buf : & mut ReadBuf <'_ >,) -> Poll < io :: Result < () >> { Pin :: new (& mut ** self) . poll_read (cx , buf) } } ; }
};
}
