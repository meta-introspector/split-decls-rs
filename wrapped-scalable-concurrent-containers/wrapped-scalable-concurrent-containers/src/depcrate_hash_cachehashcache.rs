// Generated macro for HashCache (struct)
macro_rules! Depcrate_hash_cacheHashCache {
() => {
// Module: crate::hash_cache
// Provides: {"HashCache"}
// Dependencies: {}
# [doc = " Scalable concurrent 32-way associative cache backed by [`HashMap`](super::HashMap)."] # [doc = ""] # [doc = " [`HashCache`] is a concurrent 32-way associative cache based on the"] # [doc = " [`HashMap`](super::HashMap) implementation. [`HashCache`] does not keep track of the least"] # [doc = " recently used entry in the entire cache. Instead, each bucket maintains a doubly linked list of"] # [doc = " occupied entries, updated on access to entries to keep track of the least recently used entries"] # [doc = " within the bucket. Therefore, entries can be evicted before the cache is full."] # [doc = ""] # [doc = " [`HashCache`] and [`HashMap`](super::HashMap) share the same runtime characteristics, except"] # [doc = " that each entry in a [`HashCache`] additionally uses 2-byte space for a doubly linked list and a"] # [doc = " [`HashCache`] starts evicting least recently used entries if the bucket is full instead of"] # [doc = " allocating a linked list of entries."] # [doc = ""] # [doc = " ## Unwind safety"] # [doc = ""] # [doc = " [`HashCache`] is impervious to out-of-memory errors and panics in user-specified code under one"] # [doc = " condition: `H::Hasher::hash`, `K::drop` and `V::drop` must not panic."] pub struct HashCache < K , V , H = RandomState > where H : BuildHasher , { bucket_array : AtomicShared < BucketArray < K , V , DoublyLinkedList , CACHE > > , minimum_capacity : AtomicUsize , maximum_capacity : usize , build_hasher : H , }
};
}
