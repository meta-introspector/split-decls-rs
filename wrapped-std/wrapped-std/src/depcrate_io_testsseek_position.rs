// Generated macro for seek_position (function)
macro_rules! Depcrate_io_testsseek_position {
() => {
// Module: crate::io::tests
// Provides: {"seek_position"}
// Dependencies: {}
# [test] fn seek_position () -> io :: Result < () > { let mut c = Cursor :: new (vec ! [0 ; 15]) ; assert_eq ! (c . stream_position () ?, 0) ; assert_eq ! (c . stream_position () ?, 0) ; c . seek (SeekFrom :: End (0)) ? ; assert_eq ! (c . stream_position () ?, 15) ; assert_eq ! (c . stream_position () ?, 15) ; c . seek (SeekFrom :: Start (7)) ? ; c . seek (SeekFrom :: Current (2)) ? ; assert_eq ! (c . stream_position () ?, 9) ; assert_eq ! (c . stream_position () ?, 9) ; c . seek (SeekFrom :: End (- 3)) ? ; c . seek (SeekFrom :: Current (1)) ? ; c . seek (SeekFrom :: Current (- 5)) ? ; assert_eq ! (c . stream_position () ?, 8) ; assert_eq ! (c . stream_position () ?, 8) ; c . rewind () ? ; assert_eq ! (c . stream_position () ?, 0) ; assert_eq ! (c . stream_position () ?, 0) ; Ok (()) }
};
}
