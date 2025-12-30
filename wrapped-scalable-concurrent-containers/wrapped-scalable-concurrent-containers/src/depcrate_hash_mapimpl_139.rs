// Generated macro for impl_139 (impl)
macro_rules! Depcrate_hash_mapimpl_139 {
() => {
// Module: crate::hash_map
// Provides: {"impl_139"}
// Dependencies: {}
impl < K , V , H > FromIterator < (K , V) > for HashMap < K , V , H > where K : Eq + Hash , H : BuildHasher + Default , { # [inline] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let into_iter = iter . into_iter () ; let hashmap = Self :: with_capacity_and_hasher (Self :: capacity_from_size_hint (into_iter . size_hint ()) , H :: default () ,) ; into_iter . for_each (| e | { hashmap . upsert_sync (e . 0 , e . 1) ; }) ; hashmap } }
};
}
