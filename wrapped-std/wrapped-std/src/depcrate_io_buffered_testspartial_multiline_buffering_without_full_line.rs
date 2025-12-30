// Generated macro for partial_multiline_buffering_without_full_line (function)
macro_rules! Depcrate_io_buffered_testspartial_multiline_buffering_without_full_line {
() => {
// Module: crate::io::buffered::tests
// Provides: {"partial_multiline_buffering_without_full_line"}
// Dependencies: {}
# [doc = " Same as test_partial_multiline_buffering, but in the event NO full lines"] # [doc = " fit in the buffer, just buffer as much as possible"] # [test] fn partial_multiline_buffering_without_full_line () { let writer = ProgrammableSink { accept_prefix : Some (5) , .. Default :: default () } ; let mut writer = LineWriter :: with_capacity (5 , writer) ; let content = b"AAAAABBBBBBBBBB\nCCCCC\nDDDDD" ; assert_eq ! (writer . write (content) . unwrap () , 10) ; assert_eq ! (writer . get_ref () . buffer , * b"AAAAA") ; writer . flush () . unwrap () ; assert_eq ! (writer . get_ref () . buffer , * b"AAAAABBBBB") ; }
};
}
