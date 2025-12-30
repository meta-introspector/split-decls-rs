// Generated macro for test_bool_eof_partial (function)
macro_rules! Depcrate_binary_bits_teststest_bool_eof_partial {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_bool_eof_partial"}
// Dependencies: {}
# [test] fn test_bool_eof_partial () { let input = Partial :: new ([0b10000000] . as_ref ()) ; # [allow (clippy :: type_complexity)] let result : ModalResult < ((Partial < & [u8] > , usize) , bool) , InputError < _ > > = bool . parse_peek ((input , 8)) ; assert_eq ! (result , Err (crate :: error :: ErrMode :: Incomplete (Needed :: new (1)))) ; }
};
}
