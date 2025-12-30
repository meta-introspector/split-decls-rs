// Generated macro for test_take_complete_span_over_multiple_bytes (function)
macro_rules! Depcrate_binary_bits_teststest_take_complete_span_over_multiple_bytes {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_take_complete_span_over_multiple_bytes"}
// Dependencies: {}
# [test] fn test_take_complete_span_over_multiple_bytes () { let input = & [0b00010010 , 0b00110100 , 0b11111111 , 0b11111111] [..] ; let result : ModalResult < ((& [u8] , usize) , usize) , InputError < _ > > = take (24usize) . parse_peek ((input , 4)) ; assert_eq ! (result , Ok ((([0b11111111] . as_ref () , 4) , 0b1000110100111111111111))) ; }
};
}
