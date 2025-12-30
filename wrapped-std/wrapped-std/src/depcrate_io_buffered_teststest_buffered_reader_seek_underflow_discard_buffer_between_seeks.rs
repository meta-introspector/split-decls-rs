// Generated macro for test_buffered_reader_seek_underflow_discard_buffer_between_seeks (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_seek_underflow_discard_buffer_between_seeks {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_seek_underflow_discard_buffer_between_seeks"}
// Dependencies: {}
# [test] fn test_buffered_reader_seek_underflow_discard_buffer_between_seeks () { struct ErrAfterFirstSeekReader { first_seek : bool , } impl Read for ErrAfterFirstSeekReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { for x in & mut * buf { * x = 0 ; } Ok (buf . len ()) } } impl Seek for ErrAfterFirstSeekReader { fn seek (& mut self , _ : SeekFrom) -> io :: Result < u64 > { if self . first_seek { self . first_seek = false ; Ok (0) } else { Err (io :: Error :: new (io :: ErrorKind :: Other , "oh no!")) } } } let mut reader = BufReader :: with_capacity (5 , ErrAfterFirstSeekReader { first_seek : true }) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 0 , 0 , 0 , 0] [..])) ; assert ! (reader . seek (SeekFrom :: Current (i64 :: MIN)) . is_err ()) ; assert_eq ! (reader . buffer () . len () , 0) ; }
};
}
