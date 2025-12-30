// Generated macro for line_write_all_error (function)
macro_rules! Depcrate_io_buffered_testsline_write_all_error {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_write_all_error"}
// Dependencies: {}
# [test] fn line_write_all_error () { let writer = ProgrammableSink { accept_prefix : Some (5) , max_writes : Some (3) , .. Default :: default () } ; let mut writer = LineWriter :: new (writer) ; let res = writer . write_all (b"Line 1\nLine 2\nLine 3\nLine 4\nPartial") ; assert ! (res . is_err ()) ; }
};
}
