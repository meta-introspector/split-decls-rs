// Generated macro for impl_458 (impl)
macro_rules! Depcrate_collections_hash_setimpl_458 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_458"}
// Dependencies: {}
# [stable (feature = "hash_extend_copy" , since = "1.4.0")] impl < 'a , T , S > Extend < & 'a T > for HashSet < T , S > where T : 'a + Eq + Hash + Copy , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } # [inline] fn extend_one (& mut self , & item : & 'a T) { self . base . insert (item) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { Extend :: < T > :: extend_reserve (self , additional) } }
};
}
