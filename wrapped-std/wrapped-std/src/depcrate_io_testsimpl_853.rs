// Generated macro for impl_853 (impl)
macro_rules! Depcrate_io_testsimpl_853 {
() => {
// Module: crate::io::tests
// Provides: {"impl_853"}
// Dependencies: {}
impl Write for TestWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . write_vectored (& [IoSlice :: new (buf)]) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { let mut left = self . per_call ; let mut written = 0 ; for buf in bufs . iter () . take (self . n_bufs) { let n = min (left , buf . len ()) ; self . written . extend_from_slice (& buf [0 .. n]) ; left -= n ; written += n ; } Ok (written) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
};
}
