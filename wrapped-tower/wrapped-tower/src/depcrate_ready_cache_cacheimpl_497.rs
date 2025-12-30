// Generated macro for impl_497 (impl)
macro_rules! Depcrate_ready_cache_cacheimpl_497 {
() => {
// Module: crate::ready_cache::cache
// Provides: {"impl_497"}
// Dependencies: {}
impl < K , S , Req > fmt :: Debug for Pending < K , S , Req > where K : fmt :: Debug , S : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let Self { key , cancel , ready , _pd , } = self ; f . debug_struct ("Pending") . field ("key" , key) . field ("cancel" , cancel) . field ("ready" , ready) . finish () } }
};
}
