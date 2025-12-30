// Generated macro for bufreader_full_initialize (function)
macro_rules! Depcrate_io_buffered_testsbufreader_full_initialize {
() => {
// Module: crate::io::buffered::tests
// Provides: {"bufreader_full_initialize"}
// Dependencies: {}
# [test] fn bufreader_full_initialize () { struct OneByteReader ; impl Read for OneByteReader { fn read (& mut self , buf : & mut [u8]) -> crate :: io :: Result < usize > { if buf . len () > 0 { buf [0] = 0 ; Ok (1) } else { Ok (0) } } } let mut reader = BufReader :: new (OneByteReader) ; assert_eq ! (reader . initialized () , 0) ; let buf = reader . fill_buf () . unwrap () ; assert_eq ! (buf . len () , 1) ; assert_eq ! (reader . initialized () , reader . capacity ()) ; }
};
}
