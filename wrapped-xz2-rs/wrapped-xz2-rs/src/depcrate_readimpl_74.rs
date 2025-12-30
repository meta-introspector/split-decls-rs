// Generated macro for impl_74 (impl)
macro_rules! Depcrate_readimpl_74 {
() => {
// Module: crate::read
// Provides: {"impl_74"}
// Dependencies: {}
impl < R : Read > Read for XzDecoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
