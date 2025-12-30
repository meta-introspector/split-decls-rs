// Generated macro for read_buf_full_read (function)
macro_rules! Depcrate_io_testsread_buf_full_read {
() => {
// Module: crate::io::tests
// Provides: {"read_buf_full_read"}
// Dependencies: {}
# [test] fn read_buf_full_read () { struct FullRead ; impl Read for FullRead { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { Ok (buf . len ()) } } assert_eq ! (BufReader :: new (FullRead) . fill_buf () . unwrap () . len () , DEFAULT_BUF_SIZE) ; }
};
}
