// Generated macro for check_min_length (function)
macro_rules! Depcrate_quic_veccheck_min_length {
() => {
// Module: crate::quic_vec
// Provides: {"check_min_length"}
// Dependencies: {}
# [inline (always)] fn check_min_length (length : usize , len_len : usize) -> Result < () , Error > { if cfg ! (feature = "mls") { let min_len_len = length_encoding_bytes (length as u64) ? ; if min_len_len != len_len { return Err (Error :: InvalidVectorLength) ; } } ; Ok (()) }
};
}
