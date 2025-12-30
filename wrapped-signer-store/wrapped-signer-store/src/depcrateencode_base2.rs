// Generated macro for encode_base2 (function)
macro_rules! Depcrateencode_base2 {
() => {
// Module: crate
// Provides: {"encode_base2"}
// Dependencies: {}
# [doc = " Encodes a single boolean vector using Base2 encoding."] # [doc = ""] # [doc = " The output `Vec<u8>` is prefixed with the `Version::Base2` byte."] pub fn encode_base2 (bit_vec : & BitVec < u8 , Lsb0 >) -> Result < Vec < u8 > , EncodeError > { let num_bits = bit_vec . len () ; if num_bits > u16 :: MAX as usize { return Err (EncodeError :: LengthExceedsLimit) ; } let raw_slice = bit_vec . as_raw_slice () ; let capacity = HEADER_LEN . checked_add (raw_slice . len ()) . ok_or (EncodeError :: ArithmeticOverflow) ? ; let mut result = Vec :: with_capacity (capacity) ; result . push (Version :: Base2 as u8) ; result . extend_from_slice (& (num_bits as u16) . to_le_bytes ()) ; result . extend_from_slice (raw_slice) ; Ok (result) }
};
}
