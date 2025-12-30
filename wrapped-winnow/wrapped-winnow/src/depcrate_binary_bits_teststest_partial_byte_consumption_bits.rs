// Generated macro for test_partial_byte_consumption_bits (function)
macro_rules! Depcrate_binary_bits_teststest_partial_byte_consumption_bits {
() => {
// Module: crate::binary::bits::tests
// Provides: {"test_partial_byte_consumption_bits"}
// Dependencies: {}
# [test] # [doc = " Take the `bits` function and assert that remaining bytes are correctly returned, if the"] # [doc = " previous bytes are NOT fully consumed. Partially consumed bytes are supposed to be dropped."] # [doc = " I.e. if we consume 1.5 bytes of 4 bytes, 2 bytes will be returned, bits 13-16 will be"] # [doc = " dropped."] fn test_partial_byte_consumption_bits () { let input = & [0x12 , 0x34 , 0x56 , 0x78] [..] ; let result : ModalResult < (& [u8] , (u8 , u8)) , InputError < _ > > = bits :: < _ , _ , ErrMode < InputError < (& [u8] , usize) > > , _ , _ > ((take (4usize) , take (8usize))) . parse_peek (input) ; let output = result . expect ("We take 1.5 bytes and the input is longer than 2 bytes") ; let remaining = output . 0 ; assert_eq ! (remaining , [0x56 , 0x78]) ; let parsed = output . 1 ; assert_eq ! (parsed . 0 , 0x01) ; assert_eq ! (parsed . 1 , 0x23) ; }
};
}
