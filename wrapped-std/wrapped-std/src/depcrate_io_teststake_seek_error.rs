// Generated macro for take_seek_error (function)
macro_rules! Depcrate_io_teststake_seek_error {
() => {
// Module: crate::io::tests
// Provides: {"take_seek_error"}
// Dependencies: {}
# [test] fn take_seek_error () { let buf = Cursor :: new (b"0123456789") ; let mut take = buf . take (2) ; assert ! (take . seek (SeekFrom :: Start (3)) . is_err ()) ; assert ! (take . seek (SeekFrom :: End (1)) . is_err ()) ; assert ! (take . seek (SeekFrom :: End (- 3)) . is_err ()) ; assert ! (take . seek (SeekFrom :: Current (- 1)) . is_err ()) ; assert ! (take . seek (SeekFrom :: Current (3)) . is_err ()) ; }
};
}
