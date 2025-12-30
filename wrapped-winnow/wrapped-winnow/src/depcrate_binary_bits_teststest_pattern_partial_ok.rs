// Generated macro for test_pattern_partial_ok (function)
macro_rules! Depcrate_binary_bits_teststest_pattern_partial_ok {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_pattern_partial_ok"}
// Dependencies: {}
# [test] fn test_pattern_partial_ok () { let input = Partial :: new (& [0b00011111] [..]) ; let offset = 0usize ; let bits_to_take = 4usize ; let value_to_pattern = 0b0001 ; let result : ModalResult < ((_ , usize) , usize) , InputError < _ > > = pattern (value_to_pattern , bits_to_take) . parse_peek ((input , offset)) ; assert_eq ! (result , Ok (((input , bits_to_take) , value_to_pattern))) ; }
};
}
