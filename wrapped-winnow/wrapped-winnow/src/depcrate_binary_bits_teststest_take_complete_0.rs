// Generated macro for test_take_complete_0 (function)
macro_rules! Depcrate_binary_bits_teststest_take_complete_0 {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_take_complete_0"}
// Dependencies: {}
# [test] fn test_take_complete_0 () { let input = & [0b00010010] [..] ; let count = 0usize ; assert_eq ! (count , 0usize) ; let offset = 0usize ; let result : ModalResult < ((& [u8] , usize) , usize) , InputError < _ > > = take (count) . parse_peek ((input , offset)) ; assert_eq ! (result , Ok (((input , offset) , 0))) ; }
};
}
