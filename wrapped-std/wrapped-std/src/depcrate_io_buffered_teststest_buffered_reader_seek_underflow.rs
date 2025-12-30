// Generated macro for test_buffered_reader_seek_underflow (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_seek_underflow {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_seek_underflow"}
// Dependencies: {}
# [test] fn test_buffered_reader_seek_underflow () { struct PositionReader { pos : u64 , } impl Read for PositionReader { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let len = buf . len () ; for x in buf { * x = self . pos as u8 ; self . pos = self . pos . wrapping_add (1) ; } Ok (len) } } impl Seek for PositionReader { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { match pos { SeekFrom :: Start (n) => { self . pos = n ; } SeekFrom :: Current (n) => { self . pos = self . pos . wrapping_add (n as u64) ; } SeekFrom :: End (n) => { self . pos = u64 :: MAX . wrapping_add (n as u64) ; } } Ok (self . pos) } } let mut reader = BufReader :: with_capacity (5 , PositionReader { pos : 0 }) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1 , 2 , 3 , 4] [..])) ; assert_eq ! (reader . seek (SeekFrom :: End (- 5)) . ok () , Some (u64 :: MAX - 5)) ; assert_eq ! (reader . fill_buf () . ok () . map (| s | s . len ()) , Some (5)) ; let expected = 9223372036854775802 ; assert_eq ! (reader . seek (SeekFrom :: Current (i64 :: MIN)) . ok () , Some (expected)) ; assert_eq ! (reader . fill_buf () . ok () . map (| s | s . len ()) , Some (5)) ; assert_eq ! (reader . seek (SeekFrom :: Current (0)) . ok () , Some (expected)) ; assert_eq ! (reader . get_ref () . pos , expected) ; }
};
}
