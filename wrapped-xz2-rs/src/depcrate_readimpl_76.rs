// Generated macro for impl_76 (impl)
macro_rules! Depcrate_readimpl_76 {
() => {
// Module: crate::read
// Provides: {"impl_76"}
// Dependencies: {}
impl < W : Write + Read > Write for XzDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
