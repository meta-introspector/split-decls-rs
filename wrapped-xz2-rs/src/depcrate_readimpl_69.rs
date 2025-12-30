// Generated macro for impl_69 (impl)
macro_rules! Depcrate_readimpl_69 {
() => {
// Module: crate::read
// Provides: {"impl_69"}
// Dependencies: {}
impl < R : Read > Read for XzEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
