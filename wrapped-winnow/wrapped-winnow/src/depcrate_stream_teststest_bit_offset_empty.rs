// Generated macro for test_bit_offset_empty (function)
macro_rules! Depcrate_stream_teststest_bit_offset_empty {
() => {
// Module: crate::stream::tests
// Provides: {"test_bit_offset_empty"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn test_bit_offset_empty () { let i = (& b"" [..] , 0) ; let actual = i . offset_from (& i) ; assert_eq ! (actual , 0) ; }
};
}
