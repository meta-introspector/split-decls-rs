// Generated macro for key_schedule (function)
macro_rules! Depcrate_compresskey_schedule {
() => {
// Module: crate::compress
// Provides: {"key_schedule"}
// Dependencies: {}
# [inline (always)] fn key_schedule (x : & mut [u64 ; 8]) { x [0] = x [0] . wrapping_sub (x [7] ^ 0xA5A5_A5A5_A5A5_A5A5) ; x [1] ^= x [0] ; x [2] = x [2] . wrapping_add (x [1]) ; x [3] = x [3] . wrapping_sub (x [2] ^ ((! x [1]) << 19)) ; x [4] ^= x [3] ; x [5] = x [5] . wrapping_add (x [4]) ; x [6] = x [6] . wrapping_sub (x [5] ^ ((! x [4]) >> 23)) ; x [7] ^= x [6] ; x [0] = x [0] . wrapping_add (x [7]) ; x [1] = x [1] . wrapping_sub (x [0] ^ ((! x [7]) << 19)) ; x [2] ^= x [1] ; x [3] = x [3] . wrapping_add (x [2]) ; x [4] = x [4] . wrapping_sub (x [3] ^ ((! x [2]) >> 23)) ; x [5] ^= x [4] ; x [6] = x [6] . wrapping_add (x [5]) ; x [7] = x [7] . wrapping_sub (x [6] ^ 0x0123_4567_89AB_CDEF) ; }
};
}
