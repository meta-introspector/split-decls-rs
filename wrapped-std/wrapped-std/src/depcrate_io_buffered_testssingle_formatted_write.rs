// Generated macro for single_formatted_write (function)
macro_rules! Depcrate_io_buffered_testssingle_formatted_write {
() => {
// Module: crate::io::buffered::tests
// Provides: {"single_formatted_write"}
// Dependencies: {}
# [doc = " Test that a normal, formatted writeln only results in a single write"] # [doc = " call to the underlying writer. A naive implementation of"] # [doc = " LineWriter::write_all results in two writes: one of the buffered data,"] # [doc = " and another of the final substring in the formatted set"] # [test] fn single_formatted_write () { let writer = WriteRecorder :: default () ; let mut writer = LineWriter :: new (writer) ; writeln ! (& mut writer , "{}, {}!" , "hello" , "world") . unwrap () ; assert_eq ! (writer . get_ref () . events , [RecordedEvent :: Write ("hello, world!\n" . to_string ())]) ; }
};
}
