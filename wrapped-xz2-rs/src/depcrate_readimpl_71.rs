// Generated macro for impl_71 (impl)
macro_rules! Depcrate_readimpl_71 {
() => {
// Module: crate::read
// Provides: {"impl_71"}
// Dependencies: {}
impl < W : Write + Read > Write for XzEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
