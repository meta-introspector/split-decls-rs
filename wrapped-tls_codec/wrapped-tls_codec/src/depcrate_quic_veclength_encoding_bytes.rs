// Generated macro for length_encoding_bytes (function)
macro_rules! Depcrate_quic_veclength_encoding_bytes {
() => {
// Module: crate::quic_vec
// Provides: {"length_encoding_bytes"}
// Dependencies: {}
# [inline (always)] fn length_encoding_bytes (length : u64) -> Result < usize , Error > { if ! cfg ! (fuzzing) { debug_assert ! (length <= MAX_LEN) ; } if length > MAX_LEN { return Err (Error :: InvalidVectorLength) ; } Ok (if length <= 0x3f { 1 } else if length <= 0x3fff { 2 } else if length <= 0x3fff_ffff { 4 } else { 8 }) }
};
}
