// Generated macro for test_offset_str (function)
macro_rules! Depcrate_stream_teststest_offset_str {
() => {
// Module: crate::stream::tests
// Provides: {"test_offset_str"}
// Dependencies: {}
# [test] fn test_offset_str () { let a = "abcřèÂßÇd123" ; let b = & a [7 ..] ; let c = & a [.. 5] ; let d = & a [5 .. 9] ; assert_eq ! (b . offset_from (& a) , 7) ; assert_eq ! (c . offset_from (& a) , 0) ; assert_eq ! (d . offset_from (& a) , 5) ; }
};
}
