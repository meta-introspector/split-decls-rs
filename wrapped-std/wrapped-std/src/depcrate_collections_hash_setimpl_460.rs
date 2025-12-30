// Generated macro for impl_460 (impl)
macro_rules! Depcrate_collections_hash_setimpl_460 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_460"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > BitOr < & HashSet < T , S > > for & HashSet < T , S > where T : Eq + Hash + Clone , S : BuildHasher + Default , { type Output = HashSet < T , S > ; # [doc = " Returns the union of `self` and `rhs` as a new `HashSet<T, S>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::HashSet;"] # [doc = ""] # [doc = " let a = HashSet::from([1, 2, 3]);"] # [doc = " let b = HashSet::from([3, 4, 5]);"] # [doc = ""] # [doc = " let set = &a | &b;"] # [doc = ""] # [doc = " let mut i = 0;"] # [doc = " let expected = [1, 2, 3, 4, 5];"] # [doc = " for x in &set {"] # [doc = "     assert!(expected.contains(x));"] # [doc = "     i += 1;"] # [doc = " }"] # [doc = " assert_eq!(i, expected.len());"] # [doc = " ```"] fn bitor (self , rhs : & HashSet < T , S >) -> HashSet < T , S > { self . union (rhs) . cloned () . collect () } }
};
}
