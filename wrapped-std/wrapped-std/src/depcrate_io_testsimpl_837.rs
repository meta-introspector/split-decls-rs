// Generated macro for impl_837 (impl)
macro_rules! Depcrate_io_testsimpl_837 {
() => {
// Module: crate::io::tests
// Provides: {"impl_837"}
// Dependencies: {}
impl Read for ExampleHugeRangeOfZeroes { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let max = buf . len () . min (usize :: MAX) ; for i in 0 .. max { if self . position == u64 :: MAX { return Ok (i) ; } self . position += 1 ; buf [i] = 0 ; } Ok (max) } }
};
}
