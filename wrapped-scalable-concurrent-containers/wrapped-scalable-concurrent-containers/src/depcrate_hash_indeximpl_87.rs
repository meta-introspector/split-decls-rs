// Generated macro for impl_87 (impl)
macro_rules! Depcrate_hash_indeximpl_87 {
() => {
// Module: crate::hash_index
// Provides: {"impl_87"}
// Dependencies: {}
impl < K , V , H > FromIterator < (K , V) > for HashIndex < K , V , H > where K : Eq + Hash , H : BuildHasher + Default , { # [inline] fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let into_iter = iter . into_iter () ; let hashindex = Self :: with_capacity_and_hasher (Self :: capacity_from_size_hint (into_iter . size_hint ()) , H :: default () ,) ; into_iter . for_each (| e | { let _result = hashindex . insert_sync (e . 0 , e . 1) ; }) ; hashindex } }
};
}
