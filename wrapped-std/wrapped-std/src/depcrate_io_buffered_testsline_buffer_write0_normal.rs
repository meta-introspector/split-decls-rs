// Generated macro for line_buffer_write0_normal (function)
macro_rules! Depcrate_io_buffered_testsline_buffer_write0_normal {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_buffer_write0_normal"}
// Dependencies: {}
# [doc = " Test that, if a write returns Ok(0) after a successful pre-flush, this"] # [doc = " is propagated as Ok(0)"] # [test] fn line_buffer_write0_normal () { let writer = ProgrammableSink { max_writes : Some (2) , .. Default :: default () } ; let mut writer = LineWriter :: new (writer) ; assert_eq ! (writer . write (b"Line 1\nPartial") . unwrap () , 14) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\n") ; assert_eq ! (writer . write (b" Line End\n") . unwrap () , 0) ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\nPartial") ; }
};
}
