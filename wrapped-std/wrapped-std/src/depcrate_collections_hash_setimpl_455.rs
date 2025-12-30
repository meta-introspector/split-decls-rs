// Generated macro for impl_455 (impl)
macro_rules! Depcrate_collections_hash_setimpl_455 {
() => {
// Module: crate::collections::hash::set
// Provides: {"impl_455"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , S > FromIterator < T > for HashSet < T , S > where T : Eq + Hash , S : BuildHasher + Default , { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> HashSet < T , S > { let mut set = HashSet :: with_hasher (Default :: default ()) ; set . extend (iter) ; set } }
};
}
