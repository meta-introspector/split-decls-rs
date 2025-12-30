// Generated macro for impl_50 (impl)
macro_rules! Depcrate_bufreadimpl_50 {
() => {
// Module: crate::bufread
// Provides: {"impl_50"}
// Dependencies: {}
impl < W : Write > Write for XzEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
