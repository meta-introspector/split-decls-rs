// Generated macro for impl_457 (impl)
macro_rules! Depcrate_collections_hash_setimpl_457 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_457"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > Extend < T > for HashSet < T , S > where T : Eq + Hash , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . base . extend (iter) ; } # [inline] fn extend_one (& mut self , item : T) { self . base . insert (item) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { self . base . extend_reserve (additional) ; } }
};
}
