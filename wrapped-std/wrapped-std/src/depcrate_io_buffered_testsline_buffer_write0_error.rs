// Generated macro for line_buffer_write0_error (function)
macro_rules! Depcrate_io_buffered_testsline_buffer_write0_error {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_buffer_write0_error"}
// Dependencies: {}
# [doc = " Test that, if an attempt to pre-flush buffered data returns Ok(0),"] # [doc = " this is propagated as an error."] # [test] fn line_buffer_write0_error () { let writer = ProgrammableSink { max_writes : Some (1) , .. Default :: default () } ; let mut writer = LineWriter :: new (writer) ; assert_eq ! (writer . write (b"Line 1\nPartial") . unwrap () , 14) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\n") ; let err = writer . write (b" Line End\n") . unwrap_err () ; assert_eq ! (err . kind () , ErrorKind :: WriteZero) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\n") ; }
};
}
