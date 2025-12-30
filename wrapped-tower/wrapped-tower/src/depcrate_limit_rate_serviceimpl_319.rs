// Generated macro for impl_319 (impl)
macro_rules! Depcrate_limit_rate_serviceimpl_319 {
() => {
// Module: crate::limit::rate::service
// Provides: {"impl_319"}
// Dependencies: {}
# [cfg (feature = "load")] impl < S > crate :: load :: Load for RateLimit < S > where S : crate :: load :: Load , { type Metric = S :: Metric ; fn load (& self) -> Self :: Metric { self . inner . load () } }
};
}
