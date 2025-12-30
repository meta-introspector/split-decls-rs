// Generated macro for decode_impl_base3 (function)
macro_rules! Depcratedecode_impl_base3 {
() => {
// Module: crate
// Provides: {"decode_impl_base3"}
// Dependencies: {}
fn decode_impl_base3 (data_bytes : & [u8] , total_bits : usize) -> Result < Decoded , DecodeError > { let expected_num_chunks = total_bits . div_ceil (BASE3_SYMBOLS_PER_BYTE) ; if data_bytes . len () != expected_num_chunks { return Err (DecodeError :: CorruptDataPayload) ; } let decoded_byte_len = total_bits . div_ceil (8) ; let mut base_bytes = vec ! [0u8 ; decoded_byte_len] ; let mut fallback_bytes = vec ! [0u8 ; decoded_byte_len] ; for (chunk_index , & block_byte) in data_bytes . iter () . enumerate () { let mut block_num = block_byte ; let start_bit = chunk_index . checked_mul (BASE3_SYMBOLS_PER_BYTE) . ok_or (DecodeError :: ArithmeticOverflow) ? ; let end_bit = start_bit . checked_add (BASE3_SYMBOLS_PER_BYTE) . ok_or (DecodeError :: ArithmeticOverflow) ? . min (total_bits) ; for bit_index in start_bit .. end_bit { let remainder = block_num % 3 ; block_num /= 3 ; let byte_idx = bit_index / 8 ; let bit_idx = bit_index % 8 ; let (base_bit , fallback_bit) = match remainder { 0 => (false , false) , 1 => (true , false) , 2 => (false , true) , _ => unreachable ! () , } ; if base_bit { base_bytes [byte_idx] |= 1 << bit_idx ; } if fallback_bit { fallback_bytes [byte_idx] |= 1 << bit_idx ; } } } let mut base_vec = BitVec :: from_vec (base_bytes) ; base_vec . truncate (total_bits) ; let mut fallback_vec = BitVec :: from_vec (fallback_bytes) ; fallback_vec . truncate (total_bits) ; Ok (Decoded :: Base3 (base_vec , fallback_vec)) }
};
}
