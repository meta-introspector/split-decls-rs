// Generated macro for impl_56 (impl)
macro_rules! Depcrate_bufreadimpl_56 {
() => {
// Module: crate::bufread
// Provides: {"impl_56"}
// Dependencies: {}
impl < W : Write > Write for XzDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
