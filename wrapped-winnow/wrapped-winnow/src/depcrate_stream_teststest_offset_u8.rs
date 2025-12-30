// Generated macro for test_offset_u8 (function)
macro_rules! Depcrate_stream_teststest_offset_u8 {
() => {
// Module: crate::stream::tests
// Provides: {"test_offset_u8"}
// Dependencies: {}
# [test] fn test_offset_u8 () { let s = b"abcd123" ; let a = & s [..] ; let b = & a [2 ..] ; let c = & a [.. 4] ; let d = & a [3 .. 5] ; assert_eq ! (b . offset_from (& a) , 2) ; assert_eq ! (c . offset_from (& a) , 0) ; assert_eq ! (d . offset_from (& a) , 3) ; }
};
}
