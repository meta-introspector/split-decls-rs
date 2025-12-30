// Generated macro for test_take_partial_0 (function)
macro_rules! Depcrate_binary_bits_teststest_take_partial_0 {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_take_partial_0"}
// Dependencies: {}
# [test] fn test_take_partial_0 () { let input = Partial :: new (& [] [..]) ; let count = 0usize ; assert_eq ! (count , 0usize) ; let offset = 0usize ; let result : ModalResult < ((_ , usize) , usize) , InputError < _ > > = take (count) . parse_peek ((input , offset)) ; assert_eq ! (result , Ok (((input , offset) , 0))) ; }
};
}
