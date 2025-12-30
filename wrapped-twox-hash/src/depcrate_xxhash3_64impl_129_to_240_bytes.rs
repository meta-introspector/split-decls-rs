// Generated macro for impl_129_to_240_bytes (function)
macro_rules! Depcrate_xxhash3_64impl_129_to_240_bytes {
() => {
// Module: crate::xxhash3_64
// Provides: {"impl_129_to_240_bytes"}
// Dependencies: {}
# [inline] fn impl_129_to_240_bytes (secret : & Secret , seed : u64 , input : & [u8]) -> u64 { assert_input_range ! (129 ..= 240 , input . len ()) ; let mut acc = input . len () . into_u64 () . wrapping_mul (PRIME64_1) ; let (head , _) = input . bp_as_chunks () ; let mut head = head . iter () ; let ss = secret . for_64 () . words_for_127_to_240_part1 () ; for (chunk , secret) in head . by_ref () . zip (ss) . take (8) { acc = acc . wrapping_add (mix_step (chunk , secret , seed)) ; } acc = avalanche (acc) ; let ss = secret . for_64 () . words_for_127_to_240_part2 () ; for (chunk , secret) in head . zip (ss) { acc = acc . wrapping_add (mix_step (chunk , secret , seed)) ; } let last_chunk = input . last_chunk () . unwrap () ; let ss = secret . for_64 () . words_for_127_to_240_part3 () ; acc = acc . wrapping_add (mix_step (last_chunk , ss , seed)) ; avalanche (acc) }
};
}
