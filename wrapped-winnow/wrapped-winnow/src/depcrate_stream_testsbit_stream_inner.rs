// Generated macro for bit_stream_inner (function)
macro_rules! Depcrate_stream_testsbit_stream_inner {
() => {
// Module: crate::stream::tests
// Provides: {"bit_stream_inner"}
// Dependencies: {}
# [cfg (feature = "std")] fn bit_stream_inner (byte_len : usize , start : usize) { let start = start . min (byte_len * 8) ; let start_byte = start / 8 ; let start_bit = start % 8 ; let bytes = vec ! [0b1010_1010 ; byte_len] ; let i = (& bytes [start_byte ..] , start_bit) ; let mut curr_i = i ; let mut curr_offset = 0 ; while let Some (_token) = curr_i . peek_token () { let to_offset = curr_i . offset_from (& i) ; assert_eq ! (curr_offset , to_offset) ; let actual_slice = i . peek_slice (curr_offset) ; let expected_slice = i . clone () . peek_slice (curr_offset) ; assert_eq ! (actual_slice , expected_slice) ; let at_offset = i . offset_at (curr_offset) . unwrap () ; assert_eq ! (curr_offset , at_offset) ; let eof_offset = curr_i . eof_offset () ; let eof_slice = curr_i . peek_slice (eof_offset) ; let eof_slice_i = (eof_slice . 0 , eof_slice . 1) ; assert_eq ! (eof_slice_i , curr_i) ; curr_offset += 1 ; let _ = curr_i . next_token () ; } assert_eq ! (i . eof_offset () , curr_offset) ; }
};
}
