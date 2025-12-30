// Generated macro for impl_41 (impl)
macro_rules! Depcrate_hash_cacheimpl_41 {
() => {
// Module: crate::hash_cache
// Provides: {"impl_41"}
// Dependencies: {}
impl < K , V , H > HashTable < K , V , H , DoublyLinkedList , CACHE > for HashCache < K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn hasher (& self) -> & H { & self . build_hasher } # [inline] fn bucket_array (& self) -> & AtomicShared < BucketArray < K , V , DoublyLinkedList , CACHE > > { & self . bucket_array } # [inline] fn minimum_capacity (& self) -> & AtomicUsize { & self . minimum_capacity } # [inline] fn maximum_capacity (& self) -> usize { self . maximum_capacity } }
};
}
