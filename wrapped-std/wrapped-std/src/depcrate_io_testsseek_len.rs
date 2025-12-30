// Generated macro for seek_len (function)
macro_rules! Depcrate_io_testsseek_len {
() => {
// Module: crate::io::tests
// Provides: {"seek_len"}
// Dependencies: {}
# [test] fn seek_len () -> io :: Result < () > { let mut c = Cursor :: new (vec ! [0 ; 15]) ; assert_eq ! (c . stream_len () ?, 15) ; c . seek (SeekFrom :: End (0)) ? ; let old_pos = c . stream_position () ? ; assert_eq ! (c . stream_len () ?, 15) ; assert_eq ! (c . stream_position () ?, old_pos) ; c . seek (SeekFrom :: Start (7)) ? ; c . seek (SeekFrom :: Current (2)) ? ; let old_pos = c . stream_position () ? ; assert_eq ! (c . stream_len () ?, 15) ; assert_eq ! (c . stream_position () ?, old_pos) ; Ok (()) }
};
}
