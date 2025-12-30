// Generated macro for impl_17_to_128_bytes (function)
macro_rules! Depcrate_xxhash3_128impl_17_to_128_bytes {
() => {
// Module: crate::xxhash3_128
// Provides: {"impl_17_to_128_bytes"}
// Dependencies: {}
# [inline] fn impl_17_to_128_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u128 { assert_input_range ! (17 ..= 128 , input . len ()) ; let input_len = input . len () . into_u64 () ; let mut acc = [input_len . wrapping_mul (PRIME64_1) , 0] ; impl_17_to_128_bytes_iter (secret , input , | fwd , bwd , secret | { mix_two_chunks (& mut acc , fwd , bwd , secret , seed) ; }) ; finalize_medium (acc , input_len , seed) }
};
}
