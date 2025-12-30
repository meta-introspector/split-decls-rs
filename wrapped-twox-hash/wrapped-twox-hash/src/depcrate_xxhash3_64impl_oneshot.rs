// Generated macro for impl_oneshot (function)
macro_rules! Depcrate_xxhash3_64impl_oneshot {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_oneshot"}
// Dependencies: {}
# [inline (always)] fn impl_oneshot (secret : & Secret , seed : u64 , input : & [u8]) -> u64 { match input . len () { 241 .. => impl_241_plus_bytes (secret , input) , 129 ..= 240 => impl_129_to_240_bytes (secret , seed , input) , 17 ..= 128 => impl_17_to_128_bytes (secret , seed , input) , 9 ..= 16 => impl_9_to_16_bytes (secret , seed , input) , 4 ..= 8 => impl_4_to_8_bytes (secret , seed , input) , 1 ..= 3 => impl_1_to_3_bytes (secret , seed , input) , 0 => impl_0_bytes (secret , seed) , } }
};
}
