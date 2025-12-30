// Generated macro for impl_452 (impl)
macro_rules! Depcrate_collections_hash_setimpl_452 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_452"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > PartialEq for HashSet < T , S > where T : Eq + Hash , S : BuildHasher , { fn eq (& self , other : & HashSet < T , S >) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| key | other . contains (key)) } }
};
}
