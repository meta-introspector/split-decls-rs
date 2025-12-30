// Generated macro for impl_490 (impl)
macro_rules! Depcrate_ready_cache_cacheimpl_490 {
() => {
// Module: crate::ready_cache::cache
// Provides: {"impl_490"}
// Dependencies: {}
impl < K , S , Req > Default for ReadyCache < K , S , Req > where K : Eq + Hash , S : Service < Req > , { fn default () -> Self { Self { ready : IndexMap :: default () , pending : FuturesUnordered :: new () , pending_cancel_txs : IndexMap :: default () , } } }
};
}
