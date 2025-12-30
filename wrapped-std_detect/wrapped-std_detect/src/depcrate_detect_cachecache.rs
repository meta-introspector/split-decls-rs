// Generated macro for Cache (struct)
macro_rules! Depcrate_detect_cacheCache {
() => {
// Module: crate::detect::cache
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " Feature cache with capacity for `size_of::<usize>() * 8 - 1` features."] # [doc = ""] # [doc = " Note: 0 is used to represent an uninitialized cache, and (at least) the most"] # [doc = " significant bit is set on any cache which has been initialized."] # [doc = ""] # [doc = " Note: we use `Relaxed` atomic operations, because we are only interested in"] # [doc = " the effects of operations on a single memory location. That is, we only need"] # [doc = " \"modification order\", and not the full-blown \"happens before\"."] struct Cache (AtomicUsize) ;
};
}
