// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < S > Write for AllowStd < S > where S : AsyncWrite + Unpin , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . with_context (| ctx , stream | stream . poll_write (ctx , buf)) } fn flush (& mut self) -> io :: Result < () > { self . with_context (| ctx , stream | stream . poll_flush (ctx)) } }
};
}
