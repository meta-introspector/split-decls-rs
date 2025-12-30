// Generated macro for line_write_all (function)
macro_rules! Depcrate_io_buffered_testsline_write_all {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_write_all"}
// Dependencies: {}
# [doc = " LineWriter has a custom `write_all`; make sure it works correctly"] # [test] fn line_write_all () { let writer = ProgrammableSink { accept_prefix : Some (5) , .. Default :: default () } ; let mut writer = LineWriter :: new (writer) ; writer . write_all (b"Line 1\nLine 2\nLine 3\nLine 4\nPartial") . unwrap () ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nLine 2\nLine 3\nLine 4\n") ; writer . write_all (b" Line 5\n") . unwrap () ; assert_eq ! (writer . get_ref () . buffer . as_slice () , b"Line 1\nLine 2\nLine 3\nLine 4\nPartial Line 5\n" . as_ref () ,) ; }
};
}
