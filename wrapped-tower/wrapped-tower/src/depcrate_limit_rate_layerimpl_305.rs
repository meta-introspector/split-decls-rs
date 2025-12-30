// Generated macro for impl_305 (impl)
macro_rules! Depcrate_limit_rate_layerimpl_305 {
() => {
// Module: crate::limit::rate::layer
// Provides: {"impl_305"}
// Dependencies: {}
impl < S > Layer < S > for RateLimitLayer { type Service = RateLimit < S > ; fn layer (& self , service : S) -> Self :: Service { RateLimit :: new (service , self . rate) } }
};
}
