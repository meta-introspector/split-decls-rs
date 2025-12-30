// Generated macro for CONFIG (static)
macro_rules! Depcrate_interest_cacheCONFIG {
() => {
// Module: crate::interest_cache
// Provides: {"CONFIG"}
// Dependencies: {}
static CONFIG : Lazy < Mutex < InterestCacheConfig > > = Lazy :: new (| | { tracing_core :: callsite :: register (& SENTINEL_CALLSITE) ; Mutex :: new (InterestCacheConfig :: disabled ()) }) ;
};
}
