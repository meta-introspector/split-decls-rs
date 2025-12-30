// Generated macro for impl_97 (impl)
macro_rules! Depcrate_writeimpl_97 {
() => {
// Module: crate::write
// Provides: {"impl_97"}
// Dependencies: {}
impl < W : Read + Write > Read for XzDecoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
};
}
