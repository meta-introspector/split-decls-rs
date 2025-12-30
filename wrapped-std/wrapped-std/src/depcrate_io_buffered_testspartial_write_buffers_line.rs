// Generated macro for partial_write_buffers_line (function)
macro_rules! Depcrate_io_buffered_testspartial_write_buffers_line {
() => {
// Module: crate::io::buffered::tests
// Provides: {"partial_write_buffers_line"}
// Dependencies: {}
# [doc = " Test that, given this input:"] # [doc = ""] # [doc = " Line 1\\n"] # [doc = " Line 2\\n"] # [doc = " Line 3\\n"] # [doc = " Line 4"] # [doc = ""] # [doc = " And given a result that only writes to midway through Line 2"] # [doc = ""] # [doc = " That only up to the end of Line 3 is buffered"] # [doc = ""] # [doc = " This behavior is desirable because it prevents flushing partial lines"] # [test] fn partial_write_buffers_line () { let writer = ProgrammableSink { accept_prefix : Some (13) , .. Default :: default () } ; let mut writer = LineWriter :: new (writer) ; assert_eq ! (writer . write (b"Line 1\nLine 2\nLine 3\nLine4") . unwrap () , 21) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2") ; assert_eq ! (writer . write (b"Line 4") . unwrap () , 6) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2\nLine 3\n") ; }
};
}
