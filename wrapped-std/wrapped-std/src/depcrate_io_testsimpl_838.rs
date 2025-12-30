// Generated macro for impl_838 (impl)
macro_rules! Depcrate_io_testsimpl_838 {
() => {
// Module: crate::io::tests
// Provides: {"impl_838"}
// Dependencies: {}
impl Seek for ExampleHugeRangeOfZeroes { fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { match pos { io :: SeekFrom :: Start (i) => self . position = i , io :: SeekFrom :: End (i) if i >= 0 => self . position = u64 :: MAX , io :: SeekFrom :: End (i) => self . position = self . position - i . unsigned_abs () , io :: SeekFrom :: Current (i) => { self . position = if i >= 0 { self . position . saturating_add (i . unsigned_abs ()) } else { self . position . saturating_sub (i . unsigned_abs ()) } ; } } Ok (self . position) } }
};
}
