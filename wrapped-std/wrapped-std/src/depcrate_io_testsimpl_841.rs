// Generated macro for impl_841 (impl)
macro_rules! Depcrate_io_testsimpl_841 {
() => {
// Module: crate::io::tests
// Provides: {"impl_841"}
// Dependencies: {}
impl < 'a > Read for ExampleSliceReader < 'a > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let len = cmp :: min (self . slice . len () , buf . len ()) ; buf [.. len] . copy_from_slice (& self . slice [.. len]) ; self . slice = & self . slice [len ..] ; Ok (len) } }
};
}
