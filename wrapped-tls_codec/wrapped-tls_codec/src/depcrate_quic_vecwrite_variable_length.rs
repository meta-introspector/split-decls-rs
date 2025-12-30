// Generated macro for write_variable_length (function)
macro_rules! Depcrate_quic_vecwrite_variable_length {
() => {
// Module: crate::quic_vec
// Provides: {"write_variable_length"}
// Dependencies: {}
# [inline (always)] pub fn write_variable_length (content_length : usize) -> Result < Vec < u8 > , Error > { let len_len = length_encoding_bytes (content_length . try_into () ?) ? ; if ! cfg ! (fuzzing) { debug_assert ! (len_len <= 8 , "Invalid vector len_len {len_len}") ; } if len_len > 8 { return Err (Error :: LibraryError) ; } let mut length_bytes = vec ! [0u8 ; len_len] ; match len_len { 1 => length_bytes [0] = 0x00 , 2 => length_bytes [0] = 0x40 , 4 => length_bytes [0] = 0x80 , 8 => length_bytes [0] = 0xc0 , _ => { if ! cfg ! (fuzzing) { debug_assert ! (false , "Invalid vector len_len {len_len}") ; } return Err (Error :: InvalidVectorLength) ; } } let mut len = content_length ; for b in length_bytes . iter_mut () . rev () { * b |= (len & 0xFF) as u8 ; len >>= 8 ; } Ok (length_bytes) }
};
}
