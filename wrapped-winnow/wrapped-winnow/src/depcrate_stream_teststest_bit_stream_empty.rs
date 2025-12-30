// Generated macro for test_bit_stream_empty (function)
macro_rules! Depcrate_stream_teststest_bit_stream_empty {
() => {
// Module: crate::stream::tests
// Provides: {"test_bit_stream_empty"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn test_bit_stream_empty () { let i = (& b"" [..] , 0) ; let actual = i . iter_offsets () . collect :: < alloc :: vec :: Vec < _ > > () ; assert_eq ! (actual , vec ! []) ; let actual = i . eof_offset () ; assert_eq ! (actual , 0) ; let actual = i . peek_token () ; assert_eq ! (actual , None) ; let actual = i . offset_for (| b | b) ; assert_eq ! (actual , None) ; let actual = i . offset_at (1) ; assert_eq ! (actual , Err (Needed :: new (1))) ; let actual_slice = i . peek_slice (0) ; assert_eq ! (actual_slice , (& b"" [..] , 0 , 0)) ; }
};
}
