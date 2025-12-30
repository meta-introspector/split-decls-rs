// Generated macro for test_take_complete_eof (function)
macro_rules! Depcrate_binary_bits_teststest_take_complete_eof {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_take_complete_eof"}
// Dependencies: {}
# [test] fn test_take_complete_eof () { let input = & [0b00010010] [..] ; let result : ModalResult < ((& [u8] , usize) , usize) , InputError < _ > > = take (1usize) . parse_peek ((input , 8)) ; assert_eq ! (result , Err (crate :: error :: ErrMode :: Backtrack (InputError :: at ((input , 8) ,)))) ; }
};
}
