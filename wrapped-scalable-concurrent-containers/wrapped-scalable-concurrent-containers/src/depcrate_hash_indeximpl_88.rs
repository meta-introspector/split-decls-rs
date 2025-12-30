// Generated macro for impl_88 (impl)
macro_rules! Depcrate_hash_indeximpl_88 {
() => {
// Module: crate::hash_index
// Provides: {"impl_88"}
// Dependencies: {}
impl < K , V , H > HashTable < K , V , H , () , INDEX > for HashIndex < K , V , H > where K : Eq + Hash , H : BuildHasher , { # [inline] fn hasher (& self) -> & H { & self . build_hasher } # [inline] fn defer_reclaim (& self , bucket_array : Shared < BucketArray < K , V , () , INDEX > > , guard : & Guard) { self . reclaim_memory (guard) ; self . garbage_epoch . swap (u8 :: from (guard . epoch ()) , Release) ; let (Some (prev_head) , _) = self . garbage_chain . swap ((Some (bucket_array . clone ()) , Tag :: None) , AcqRel) else { return ; } ; bucket_array . bucket_link () . swap ((Some (prev_head) , Tag :: None) , Release) ; } # [inline] fn bucket_array (& self) -> & AtomicShared < BucketArray < K , V , () , INDEX > > { & self . bucket_array } # [inline] fn minimum_capacity (& self) -> & AtomicUsize { & self . minimum_capacity } }
};
}
