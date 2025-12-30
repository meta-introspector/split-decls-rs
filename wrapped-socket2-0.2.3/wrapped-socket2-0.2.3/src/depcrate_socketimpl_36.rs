// Generated macro for impl_36 (impl)
macro_rules! Depcrate_socketimpl_36 {
() => {
// Module: crate::socket
// Provides: {"impl_36"}
// Dependencies: {}
impl Read for Socket { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
