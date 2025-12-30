// Generated macro for impl_431 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_431 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_431"}
// Dependencies: {}
# [doc = " Inserts all new key-values from the iterator and replaces values with existing"] # [doc = " keys with new values returned from the iterator."] # [stable (feature = "rust1" , since = "1.0.0")] impl < K , V , S > Extend < (K , V) > for HashMap < K , V , S > where K : Eq + Hash , S : BuildHasher , { # [inline] fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { self . base . extend (iter) } # [inline] fn extend_one (& mut self , (k , v) : (K , V)) { self . base . insert (k , v) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . base . extend_reserve (additional) ; } }
};
}
