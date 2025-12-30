// Generated macro for impl_140 (impl)
macro_rules! Depcrate_hash_mapimpl_140 {
() => {
// Module: crate::hash_map
// Provides: {"impl_140"}
// Dependencies: {}
impl < K , V , H > HashTable < K , V , H , () , MAP > for HashMap < K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn hasher (& self) -> & H { & self . build_hasher } # [inline] fn bucket_array (& self) -> & AtomicShared < BucketArray < K , V , () , MAP > > { & self . bucket_array } # [inline] fn minimum_capacity (& self) -> & AtomicUsize { & self . minimum_capacity } }
};
}
