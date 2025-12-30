// Generated macro for impl_17_to_128_bytes (function)
macro_rules! Depcrate_xxhash3_64impl_17_to_128_bytes {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_17_to_128_bytes"}
// Dependencies: {}
# [inline] fn impl_17_to_128_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u64 { assert_input_range ! (17 ..= 128 , input . len ()) ; let mut acc = input . len () . into_u64 () . wrapping_mul (PRIME64_1) ; impl_17_to_128_bytes_iter (secret , input , | fwd , bwd , secret | { acc = acc . wrapping_add (mix_step (fwd , & secret [0] , seed)) ; acc = acc . wrapping_add (mix_step (bwd , & secret [1] , seed)) ; }) ; avalanche (acc) }
};
}
