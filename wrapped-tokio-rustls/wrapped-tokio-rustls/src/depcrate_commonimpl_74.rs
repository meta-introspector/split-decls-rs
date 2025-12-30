// Generated macro for impl_74 (impl)
macro_rules! Depcrate_commonimpl_74 {
() => {
// Module: crate::common
// Provides: {"impl_74"}
// Dependencies: {}
impl < T : AsyncWrite + Unpin > Write for SyncWriteAdapter < '_ , '_ , T > { # [inline] fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . poll_with (| io , cx | io . poll_write (cx , buf)) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . poll_with (| io , cx | io . poll_write_vectored (cx , bufs)) } fn flush (& mut self) -> io :: Result < () > { self . poll_with (| io , cx | io . poll_flush (cx)) } }
};
}
