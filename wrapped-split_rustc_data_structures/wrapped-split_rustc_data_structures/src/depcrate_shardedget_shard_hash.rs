// Generated macro for get_shard_hash (function)
macro_rules! Depcrate_shardedget_shard_hash {
() => {
// Module: crate::sharded
// Provides: {"get_shard_hash"}
// Dependencies: {}
# [doc = " Get a shard with a pre-computed hash value. If `get_shard_by_value` is"] # [doc = " ever used in combination with `get_shard_by_hash` on a single `Sharded`"] # [doc = " instance, then `hash` must be computed with `FxHasher`. Otherwise,"] # [doc = " `hash` can be computed with any hasher, so long as that hasher is used"] # [doc = " consistently for each `Sharded` instance."] # [inline] fn get_shard_hash (hash : u64) -> usize { let hash_len = size_of :: < usize > () ; (hash >> (hash_len * 8 - 7 - SHARD_BITS)) as usize }
};
}
