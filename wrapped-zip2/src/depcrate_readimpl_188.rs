// Generated macro for impl_188 (impl)
macro_rules! Depcrate_readimpl_188 {
() => {
// Module: crate::read
// Provides: {"impl_188"}
// Dependencies: {}
impl < R : Read > Read for ZipFileSeek < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match & mut self . reader { ZipFileSeekReader :: Raw (r) => r . read (buf) , } } }
};
}
