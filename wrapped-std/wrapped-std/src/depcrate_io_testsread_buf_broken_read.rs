// Generated macro for read_buf_broken_read (function)
macro_rules! Depcrate_io_testsread_buf_broken_read {
() => {
// Module: crate::io::tests
// Provides: {"read_buf_broken_read"}
// Dependencies: {}
# [test] # [should_panic] fn read_buf_broken_read () { struct MalformedRead ; impl Read for MalformedRead { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { Ok (buf . len () + 1) } } let _ = BufReader :: new (MalformedRead) . fill_buf () ; }
};
}
