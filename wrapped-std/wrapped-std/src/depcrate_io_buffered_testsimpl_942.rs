// Generated macro for impl_942 (impl)
macro_rules! Depcrate_io_buffered_testsimpl_942 {
() => {
// Module: crate::io::buffered::tests
// Provides: {"impl_942"}
// Dependencies: {}
impl Read for ShortReader { fn read (& mut self , _ : & mut [u8]) -> io :: Result < usize > { if self . lengths . is_empty () { Ok (0) } else { Ok (self . lengths . remove (0)) } } }
};
}
