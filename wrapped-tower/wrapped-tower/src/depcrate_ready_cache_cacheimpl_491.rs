// Generated macro for impl_491 (impl)
macro_rules! Depcrate_ready_cache_cacheimpl_491 {
() => {
// Module: crate::ready_cache::cache
// Provides: {"impl_491"}
// Dependencies: {}
impl < K , S , Req > fmt :: Debug for ReadyCache < K , S , Req > where K : fmt :: Debug + Eq + Hash , S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Self { pending , pending_cancel_txs , ready , } = self ; f . debug_struct ("ReadyCache") . field ("pending" , pending) . field ("pending_cancel_txs" , pending_cancel_txs) . field ("ready" , ready) . finish () } }
};
}
