// Generated macro for calculate_length (function)
macro_rules! Depcrate_quic_veccalculate_length {
() => {
// Module: crate::quic_vec
// Provides: {"calculate_length"}
// Dependencies: {}
# [inline (always)] fn calculate_length (len_len_byte : u8) -> Result < (usize , usize) , Error > { let length : usize = (len_len_byte & 0x3F) . into () ; let len_len_log = (len_len_byte >> 6) . into () ; if ! cfg ! (fuzzing) { debug_assert ! (len_len_log <= MAX_LEN_LEN_LOG) ; } if len_len_log > MAX_LEN_LEN_LOG { return Err (Error :: InvalidVectorLength) ; } let len_len = match len_len_log { 0 => 1 , 1 => 2 , 2 => 4 , 3 => 8 , _ => unreachable ! () , } ; Ok ((length , len_len)) }
};
}
