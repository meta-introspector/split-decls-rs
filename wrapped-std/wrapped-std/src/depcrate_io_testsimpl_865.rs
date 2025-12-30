// Generated macro for impl_865 (impl)
macro_rules! Depcrate_io_testsimpl_865 {
() => {
// Module: crate::io::tests
// Provides: {"impl_865"}
// Dependencies: {}
impl Read for DataAndErrorReader { fn read (& mut self , _buf : & mut [u8]) -> io :: Result < usize > { panic ! ("We want tests to use `read_buf`") } fn read_buf (& mut self , buf : io :: BorrowedCursor < '_ >) -> io :: Result < () > { self . 0 . read_buf (buf) . unwrap () ; Err (io :: Error :: other ("error")) } }
};
}
