// Generated macro for impl_60 (impl)
macro_rules! Depcrate_channelimpl_60 {
() => {
// Module: crate::channel
// Provides: {"impl_60"}
// Dependencies: {}
impl Write for Channel { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . stream (0) . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . stream (0) . flush () } }
};
}
