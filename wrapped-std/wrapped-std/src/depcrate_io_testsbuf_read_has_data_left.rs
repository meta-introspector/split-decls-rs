// Generated macro for buf_read_has_data_left (function)
macro_rules! Depcrate_io_testsbuf_read_has_data_left {
() => {
// Module: crate::io::tests
// Provides: {"buf_read_has_data_left"}
// Dependencies: {}
# [test] fn buf_read_has_data_left () { let mut buf = Cursor :: new (& b"abcd" [..]) ; assert ! (buf . has_data_left () . unwrap ()) ; buf . read_exact (& mut [0 ; 2]) . unwrap () ; assert ! (buf . has_data_left () . unwrap ()) ; buf . read_exact (& mut [0 ; 2]) . unwrap () ; assert ! (! buf . has_data_left () . unwrap ()) ; }
};
}
