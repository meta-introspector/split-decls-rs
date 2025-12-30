// Generated macro for impl_39 (impl)
macro_rules! Depcrate_socketimpl_39 {
() => {
// Module: crate::socket
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > Write for & 'a Socket { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { (& self . inner) . write (buf) } fn flush (& mut self) -> io :: Result < () > { (& self . inner) . flush () } }
};
}
