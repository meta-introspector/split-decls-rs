// Generated macro for HashSet (struct)
macro_rules! Depcrate_hash_setHashSet {
() => {
// Module: crate::hash_set
// Provides: {"HashSet"}
// Dependencies: {}
# [doc = " Scalable concurrent hash set."] # [doc = ""] # [doc = " [`HashSet`] is a concurrent hash set based on [`HashMap`]."] pub struct HashSet < K , H = RandomState > where H : BuildHasher , { map : HashMap < K , () , H > , }
};
}
