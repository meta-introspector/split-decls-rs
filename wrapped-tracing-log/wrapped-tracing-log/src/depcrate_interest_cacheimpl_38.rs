// Generated macro for impl_38 (impl)
macro_rules! Depcrate_interest_cacheimpl_38 {
() => {
// Module: crate::interest_cache
// Provides: {"impl_38"}
// Dependencies: {}
impl tracing_core :: Callsite for SentinelCallsite { fn set_interest (& self , _ : tracing_core :: subscriber :: Interest) { INTEREST_CACHE_EPOCH . fetch_add (1 , Ordering :: SeqCst) ; } fn metadata (& self) -> & tracing_core :: Metadata < '_ > { & SENTINEL_METADATA } }
};
}
