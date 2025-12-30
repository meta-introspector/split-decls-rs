// Generated macro for impl_296 (impl)
macro_rules! Depcrate_limit_concurrency_serviceimpl_296 {
() => {
// Module: crate::limit::concurrency::service
// Provides: {"impl_296"}
// Dependencies: {}
# [cfg (feature = "load")] impl < S > crate :: load :: Load for ConcurrencyLimit < S > where S : crate :: load :: Load , { type Metric = S :: Metric ; fn load (& self) -> Self :: Metric { self . inner . load () } }
};
}
