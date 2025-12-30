// Generated macro for impl_344 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_344 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_344"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > PartialEq for HashMap < K , V , S > where K : Eq + Hash , V : PartialEq , S : BuildHasher , { fn eq (& self , other : & HashMap < K , V , S >) -> bool { if self . len () != other . len () { return false ; } self . iter () . all (| (key , value) | other . get (key) . map_or (false , | v | * value == * v)) } }
};
}
