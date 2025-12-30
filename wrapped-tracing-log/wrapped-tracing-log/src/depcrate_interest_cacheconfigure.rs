// Generated macro for configure (function)
macro_rules! Depcrate_interest_cacheconfigure {
() => {
// Module: crate::interest_cache
// Provides: {"configure"}
// Dependencies: {}
pub (crate) fn configure (new_config : Option < InterestCacheConfig >) { * CONFIG . lock () . unwrap () = new_config . unwrap_or_else (InterestCacheConfig :: disabled) ; INTEREST_CACHE_EPOCH . fetch_add (1 , Ordering :: SeqCst) ; }
};
}
