// Generated macro for line_full_buffer_flushed (function)
macro_rules! Depcrate_io_buffered_testsline_full_buffer_flushed {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_full_buffer_flushed"}
// Dependencies: {}
# [test] fn line_full_buffer_flushed () { let writer = ProgrammableSink :: default () ; let mut writer = LineWriter :: with_capacity (5 , writer) ; assert_eq ! (writer . write (b"01234") . unwrap () , 5) ; assert_eq ! (writer . write (b"5") . unwrap () , 1) ; assert_eq ! (& writer . get_ref () . buffer , b"01234") ; }
};
}
