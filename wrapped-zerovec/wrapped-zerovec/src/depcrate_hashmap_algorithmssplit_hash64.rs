// Generated macro for split_hash64 (function)
macro_rules! Depcrate_hashmap_algorithmssplit_hash64 {
() => {
// Module: crate::hashmap::algorithms
// Provides: {"split_hash64"}
// Dependencies: {}
# [doc = " Split the 64bit `hash` into (g, f0, f1)."] # [doc = ""] # [doc = " g denotes the highest 16bits of the hash modulo `m`, and is referred to as first level hash."] # [doc = " (f0, f1) denotes the middle, and lower 24bits of the hash respectively."] # [doc = " (f0, f1) are used to distribute the keys with same g, into distinct slots."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `hash` - The hash to split."] # [doc = " * `m` - The modulo used to split the hash."] pub const fn split_hash64 (hash : u64 , m : usize) -> (usize , u32 , u32) { (((hash >> 48) as usize % m) , ((hash >> 24) as u32 & 0xffffff) , ((hash & 0xffffff) as u32) ,) }
};
}
