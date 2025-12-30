// Generated macro for test_bool_0_complete (function)
macro_rules! Depcrate_binary_bits_teststest_bool_0_complete {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_bool_0_complete"}
// Dependencies: {}
# [test] fn test_bool_0_complete () { let input = [0b10000000] . as_ref () ; let result : ModalResult < ((& [u8] , usize) , bool) , InputError < _ > > = bool . parse_peek ((input , 0)) ; assert_eq ! (result , Ok (((input , 1) , true))) ; }
};
}
