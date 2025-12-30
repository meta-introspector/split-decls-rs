// Generated macro for test_partial_complete (function)
macro_rules! Depcrate_stream_teststest_partial_complete {
() => {
// Module: crate::stream::tests
// Provides: {"test_partial_complete"}
// Dependencies: {}
# [test] fn test_partial_complete () { let mut i = Partial :: new (& b"" [..]) ; assert ! (Partial ::<& [u8] >:: is_partial_supported ()) ; assert ! (i . is_partial () , "incomplete by default") ; let incomplete_state = i . complete () ; assert ! (! i . is_partial () , "the stream should be marked as complete") ; i . restore_partial (incomplete_state) ; assert ! (i . is_partial () , "incomplete stream state should be restored") ; }
};
}
