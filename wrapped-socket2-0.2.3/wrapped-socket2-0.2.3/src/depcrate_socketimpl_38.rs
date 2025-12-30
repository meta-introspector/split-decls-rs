// Generated macro for impl_38 (impl)
macro_rules! Depcrate_socketimpl_38 {
() => {
// Module: crate::socket
// Provides: {"impl_38"}
// Dependencies: {}
impl Write for Socket { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
