// Generated macro for impl_432 (impl)
macro_rules! Depcrate_collections_hash_mapimpl_432 {
() => {
// Module: crate::collections::hash::map
// Provides: {"impl_432"}
// Dependencies: {}
# [stable (feature = "hash_extend_copy" , since = "1.4.0")] impl < 'a , K , V , S > Extend < (& 'a K , & 'a V) > for HashMap < K , V , S > where K : Eq + Hash + Copy , V : Copy , S : BuildHasher , { # [inline] fn extend < T : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iter : T) { self . base . extend (iter) } # [inline] fn extend_one (& mut self , (& k , & v) : (& 'a K , & 'a V)) { self . base . insert (k , v) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { Extend :: < (K , V) > :: extend_reserve (self , additional) } }
};
}
