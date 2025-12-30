// Generated macro for test_bool_0_partial (function)
macro_rules! Depcrate_binary_bits_teststest_bool_0_partial {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_bool_0_partial"}
// Dependencies: {}
# [test] fn test_bool_0_partial () { let input = Partial :: new ([0b10000000] . as_ref ()) ; # [allow (clippy :: type_complexity)] let result : ModalResult < ((Partial < & [u8] > , usize) , bool) , InputError < _ > > = bool . parse_peek ((input , 0)) ; assert_eq ! (result , Ok (((input , 1) , true))) ; }
};
}
