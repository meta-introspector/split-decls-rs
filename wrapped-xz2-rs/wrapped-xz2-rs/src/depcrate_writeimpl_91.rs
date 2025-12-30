// Generated macro for impl_91 (impl)
macro_rules! Depcrate_writeimpl_91 {
() => {
// Module: crate::write
// Provides: {"impl_91"}
// Dependencies: {}
impl < W : Read + Write > Read for XzEncoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
};
}
