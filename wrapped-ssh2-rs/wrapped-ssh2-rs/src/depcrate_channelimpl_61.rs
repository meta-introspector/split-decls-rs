// Generated macro for impl_61 (impl)
macro_rules! Depcrate_channelimpl_61 {
() => {
// Module: crate::channel
// Provides: {"impl_61"}
// Dependencies: {}
impl Read for Channel { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . stream (0) . read (buf) } }
};
}
