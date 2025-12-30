// Generated macro for take_eof (function)
macro_rules! Depcrate_io_teststake_eof {
() => {
// Module: crate::io::tests
// Provides: {"take_eof"}
// Dependencies: {}
# [test] fn take_eof () { struct R ; impl Read for R { fn read (& mut self , _ : & mut [u8]) -> io :: Result < usize > { Err (io :: const_error ! (io :: ErrorKind :: Other , "")) } } impl BufRead for R { fn fill_buf (& mut self) -> io :: Result < & [u8] > { Err (io :: const_error ! (io :: ErrorKind :: Other , "")) } fn consume (& mut self , _amt : usize) { } } let mut buf = [0 ; 1] ; assert_eq ! (0 , R . take (0) . read (& mut buf) . unwrap ()) ; assert_eq ! (b"" , R . take (0) . fill_buf () . unwrap ()) ; }
};
}
