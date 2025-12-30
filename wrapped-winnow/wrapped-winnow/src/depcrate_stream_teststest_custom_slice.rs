// Generated macro for test_custom_slice (function)
macro_rules! Depcrate_stream_teststest_custom_slice {
() => {
// Module: crate::stream::tests
// Provides: {"test_custom_slice"}
// Dependencies: {}
# [test] fn test_custom_slice () { type Token = usize ; type TokenSlice < 'i > = & 'i [Token] ; let mut tokens : TokenSlice < '_ > = & [1 , 2 , 3 , 4] ; let input = & mut tokens ; let start = input . checkpoint () ; let _ = input . next_token () ; let _ = input . next_token () ; let offset = input . offset_from (& start) ; assert_eq ! (offset , 2) ; }
};
}
