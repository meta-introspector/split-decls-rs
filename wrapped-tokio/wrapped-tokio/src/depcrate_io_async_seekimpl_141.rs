// Generated macro for impl_141 (impl)
macro_rules! Depcrate_io_async_seekimpl_141 {
() => {
// Module: crate::io::async_seek
// Provides: {"impl_141"}
// Dependencies: {}
impl < T : AsRef < [u8] > + Unpin > AsyncSeek for io :: Cursor < T > { fn start_seek (mut self : Pin < & mut Self > , pos : SeekFrom) -> io :: Result < () > { io :: Seek :: seek (& mut * self , pos) . map (drop) } fn poll_complete (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < u64 > > { Poll :: Ready (Ok (self . get_mut () . position ())) } }
};
}
