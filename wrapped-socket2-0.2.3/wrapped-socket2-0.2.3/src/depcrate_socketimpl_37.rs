// Generated macro for impl_37 (impl)
macro_rules! Depcrate_socketimpl_37 {
() => {
// Module: crate::socket
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a > Read for & 'a Socket { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { (& self . inner) . read (buf) } }
};
}
