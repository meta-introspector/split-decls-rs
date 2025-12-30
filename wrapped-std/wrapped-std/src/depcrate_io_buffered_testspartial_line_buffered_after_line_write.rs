// Generated macro for partial_line_buffered_after_line_write (function)
macro_rules! Depcrate_io_buffered_testspartial_line_buffered_after_line_write {
() => {
// Module: crate::io::buffered::tests
// Provides: {"partial_line_buffered_after_line_write"}
// Dependencies: {}
# [doc = " Test that, given this input:"] # [doc = ""] # [doc = " Line 1\\n"] # [doc = " Line 2\\n"] # [doc = " Line 3"] # [doc = ""] # [doc = " And given that the full write of lines 1 and 2 was successful"] # [doc = " That data up to Line 3 is buffered"] # [test] fn partial_line_buffered_after_line_write () { let writer = ProgrammableSink :: default () ; let mut writer = LineWriter :: new (writer) ; assert_eq ! (writer . write (b"Line 1\nLine 2\nLine 3") . unwrap () , 20) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2\n") ; assert ! (writer . flush () . is_ok ()) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2\nLine 3") ; }
};
}
