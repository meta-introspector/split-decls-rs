// Generated macro for deref_async_seek (macro)
macro_rules! Depcrate_io_async_seekderef_async_seek {
() => {
// Module: crate::io::async_seek
// Provides: {"deref_async_seek"}
// Dependencies: {}
macro_rules ! deref_async_seek { () => { fn start_seek (mut self : Pin <& mut Self >, pos : SeekFrom) -> io :: Result < () > { Pin :: new (& mut ** self) . start_seek (pos) } fn poll_complete (mut self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result < u64 >> { Pin :: new (& mut ** self) . poll_complete (cx) } } ; }
};
}
