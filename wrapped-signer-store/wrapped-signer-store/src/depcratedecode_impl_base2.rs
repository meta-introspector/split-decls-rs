// Generated macro for decode_impl_base2 (function)
macro_rules! Depcratedecode_impl_base2 {
() => {
// Module: crate
// Provides: {"decode_impl_base2"}
// Dependencies: {}
fn decode_impl_base2 (data_bytes : & [u8] , total_bits : usize) -> Result < Decoded , DecodeError > { let expected_byte_len = total_bits . div_ceil (8) ; if data_bytes . len () != expected_byte_len { return Err (DecodeError :: CorruptDataPayload) ; } let mut bit_vec = BitVec :: from_slice (data_bytes) ; bit_vec . truncate (total_bits) ; Ok (Decoded :: Base2 (bit_vec)) }
};
}
