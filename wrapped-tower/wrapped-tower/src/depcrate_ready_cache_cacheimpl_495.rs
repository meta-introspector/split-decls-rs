// Generated macro for impl_495 (impl)
macro_rules! Depcrate_ready_cache_cacheimpl_495 {
() => {
// Module: crate::ready_cache::cache
// Provides: {"impl_495"}
// Dependencies: {}
impl CancelTx { fn cancel (self) { self . 0 . canceled . store (true , Ordering :: SeqCst) ; self . 0 . waker . wake () ; } }
};
}
