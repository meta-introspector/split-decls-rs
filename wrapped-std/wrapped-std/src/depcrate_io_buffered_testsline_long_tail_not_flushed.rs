// Generated macro for line_long_tail_not_flushed (function)
macro_rules! Depcrate_io_buffered_testsline_long_tail_not_flushed {
() => {
// Module: crate::io::buffered::tests
// Provides: {"line_long_tail_not_flushed"}
// Dependencies: {}
# [doc = " Test that, given a very long partial line *after* successfully"] # [doc = " flushing a complete line, no additional writes take place. This assures"] # [doc = " the property that `write` should make at-most-one attempt to write"] # [doc = " new data."] # [test] fn line_long_tail_not_flushed () { let writer = ProgrammableSink :: default () ; let mut writer = LineWriter :: with_capacity (5 , writer) ; let bytes = b"Line 1\n0123456789" ; writer . write (bytes) . unwrap () ; assert_eq ! (& writer . get_ref () . buffer , b"Line 1\n") ; }
};
}
