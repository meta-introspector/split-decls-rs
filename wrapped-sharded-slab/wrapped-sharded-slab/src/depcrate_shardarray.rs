// Generated macro for Array (struct)
macro_rules! Depcrate_shardArray {
() => {
// Module: crate::shard
// Provides: {"Array"}
// Dependencies: {}
pub (crate) struct Array < T , C : cfg :: Config > { shards : Box < [Ptr < T , C >] > , max : AtomicUsize , }
};
}
