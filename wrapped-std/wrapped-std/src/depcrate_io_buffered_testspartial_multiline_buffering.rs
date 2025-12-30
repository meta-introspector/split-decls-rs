// Generated macro for partial_multiline_buffering (function)
macro_rules! Depcrate_io_buffered_testspartial_multiline_buffering {
() => {
// Module: crate::io::buffered::tests
// Provides: {"partial_multiline_buffering"}
// Dependencies: {}
# [doc = " Under certain circumstances, the old implementation of LineWriter"] # [doc = " would try to buffer \"to the last newline\" but be forced to buffer"] # [doc = " less than that, leading to inappropriate partial line writes."] # [doc = " Regression test for that issue."] # [test] fn partial_multiline_buffering () { let writer = ProgrammableSink { accept_prefix : Some (5) , .. Default :: default () } ; let mut writer = LineWriter :: with_capacity (10 , writer) ; let content = b"AAAAABBBBB\nCCCCDDDDDD\nEEE" ; assert_eq ! (writer . write (content) . unwrap () , 11) ; assert_eq ! (writer . get_ref () . buffer , * b"AAAAA") ; writer . flush () . unwrap () ; assert_eq ! (writer . get_ref () . buffer , * b"AAAAABBBBB\n") ; }
};
}
