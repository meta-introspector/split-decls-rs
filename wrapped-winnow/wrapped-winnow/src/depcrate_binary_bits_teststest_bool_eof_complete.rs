// Generated macro for test_bool_eof_complete (function)
macro_rules! Depcrate_binary_bits_teststest_bool_eof_complete {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_bool_eof_complete"}
// Dependencies: {}
# [test] fn test_bool_eof_complete () { let input = [0b10000000] . as_ref () ; let result : ModalResult < ((& [u8] , usize) , bool) , InputError < _ > > = bool . parse_peek ((input , 8)) ; assert_eq ! (result , Err (crate :: error :: ErrMode :: Backtrack (InputError :: at ((input , 8) ,)))) ; }
};
}
