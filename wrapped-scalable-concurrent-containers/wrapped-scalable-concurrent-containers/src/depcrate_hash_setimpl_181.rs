// Generated macro for impl_181 (impl)
macro_rules! Depcrate_hash_setimpl_181 {
() => {
// Module: crate::hash_set
// Provides: {"impl_181"}
// Dependencies: {}
impl < K , H > FromIterator < K > for HashSet < K , H > where K : Eq + Hash , H : BuildHasher + Default , { # [inline] fn from_iter < T : IntoIterator < Item = K > > (iter : T) -> Self { let into_iter = iter . into_iter () ; let hashset = Self :: with_capacity_and_hasher (HashMap :: < K , () , H > :: capacity_from_size_hint (into_iter . size_hint ()) , H :: default () ,) ; into_iter . for_each (| k | { let _result = hashset . insert_sync (k) ; }) ; hashset } }
};
}
