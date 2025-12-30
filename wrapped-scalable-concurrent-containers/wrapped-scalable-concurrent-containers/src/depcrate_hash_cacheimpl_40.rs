// Generated macro for impl_40 (impl)
macro_rules! Depcrate_hash_cacheimpl_40 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_40"}
// Dependencies: {}
impl < K , V , H > FromIterator < (K , V) > for HashCache < K , V , H > where K : Eq + Hash , H : BuildHasher + Default , { # [inline] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let into_iter = iter . into_iter () ; let size_hint = into_iter . size_hint () ; let hashcache = Self :: with_capacity_and_hasher (size_hint . 0 , Self :: capacity_from_size_hint (size_hint) , H :: default () ,) ; into_iter . for_each (| e | { let _result = hashcache . put_sync (e . 0 , e . 1) ; }) ; hashcache } }
};
}
