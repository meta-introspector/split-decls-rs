// Generated macro for long_line_flushed (function)
macro_rules! Depcrate_io_buffered_testslong_line_flushed {
() => {
// Module: crate::io::buffered::tests
// Provides: {"long_line_flushed"}
// Dependencies: {}
# [doc = " Test that for calls to LineBuffer::write where the passed bytes do not contain"] # [doc = " a newline and on their own are greater in length than the internal buffer, the"] # [doc = " passed bytes are immediately written to the inner writer."] # [test] fn long_line_flushed () { let writer = ProgrammableSink :: default () ; let mut writer = LineWriter :: with_capacity (5 , writer) ; assert_eq ! (writer . write (b"0123456789") . unwrap () , 10) ; assert_eq ! (& writer . get_ref () . buffer , b"0123456789") ; }
};
}
