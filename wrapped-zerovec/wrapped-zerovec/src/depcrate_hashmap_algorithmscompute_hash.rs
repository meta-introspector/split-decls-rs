// Generated macro for compute_hash (function)
macro_rules! Depcrate_hashmap_algorithmscompute_hash {
() => {
// Module: crate::hashmap::algorithms
// Provides: {"compute_hash"}
// Dependencies: {}
# [doc = " Compute hash using [`XxHash64`]."] pub fn compute_hash < K : Hash + ? Sized > (key : & K) -> u64 { let mut hasher = XxHash64 :: with_seed (SEED) ; key . hash (& mut hasher) ; hasher . finish () }
};
}
