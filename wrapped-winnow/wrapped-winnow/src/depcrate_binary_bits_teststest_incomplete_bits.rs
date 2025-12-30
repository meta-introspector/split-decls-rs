// Generated macro for test_incomplete_bits (function)
macro_rules! Depcrate_binary_bits_teststest_incomplete_bits {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_incomplete_bits"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] # [doc = " Ensure that in Incomplete error is thrown, if too few bytes are passed for a given parser."] fn test_incomplete_bits () { let input = Partial :: new (& [0x12] [..]) ; let result : ModalResult < (_ , (u8 , u8)) , InputError < _ > > = bits :: < _ , _ , ErrMode < InputError < (_ , usize) > > , _ , _ > ((take (4usize) , take (8usize))) . parse_peek (input) ; assert ! (result . is_err ()) ; let error = result . err () . unwrap () ; assert_eq ! ("Parsing requires 2 more data" , error . to_string ()) ; }
};
}
