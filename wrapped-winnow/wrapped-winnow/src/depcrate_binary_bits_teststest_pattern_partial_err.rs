// Generated macro for test_pattern_partial_err (function)
macro_rules! Depcrate_binary_bits_teststest_pattern_partial_err {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_pattern_partial_err"}
// Dependencies: {}
# [test] fn test_pattern_partial_err () { let input = Partial :: new (& [0b00011111] [..]) ; let offset = 0usize ; let bits_to_take = 4usize ; let value_to_pattern = 0b1111 ; let result : ModalResult < ((_ , usize) , usize) , InputError < _ > > = pattern (value_to_pattern , bits_to_take) . parse_peek ((input , offset)) ; assert_eq ! (result , Err (crate :: error :: ErrMode :: Backtrack (InputError :: at ((input , offset) ,)))) ; }
};
}
