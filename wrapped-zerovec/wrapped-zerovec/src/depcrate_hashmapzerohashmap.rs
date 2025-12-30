// Generated macro for ZeroHashMap (struct)
macro_rules! Depcrate_hashmapZeroHashMap {
() => {
// Module: crate::hashmap
// Provides: {"ZeroHashMap"}
// Dependencies: {}
# [doc = " A perfect zerohashmap optimized for lookups over immutable keys."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use zerovec::ZeroHashMap;"] # [doc = ""] # [doc = " let hashmap ="] # [doc = "     ZeroHashMap::<i32, str>::from_iter([(0, \"a\"), (1, \"b\"), (2, \"c\")]);"] # [doc = " assert_eq!(hashmap.get(&0), Some(\"a\"));"] # [doc = " assert_eq!(hashmap.get(&2), Some(\"c\"));"] # [doc = " assert_eq!(hashmap.get(&4), None);"] # [doc = " ```"] # [derive (Debug)] pub struct ZeroHashMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , { # [doc = " Array of (d0, d1) which splits the keys with same first level hash into distinct"] # [doc = " slots."] # [doc = " The ith index of the array splits the keys with first level hash i."] # [doc = " If no key with first level hash is found in the original keys, (0, 0) is used as an empty"] # [doc = " placeholder."] displacements : ZeroVec < 'a , (u32 , u32) > , keys : K :: Container , values : V :: Container , }
};
}
