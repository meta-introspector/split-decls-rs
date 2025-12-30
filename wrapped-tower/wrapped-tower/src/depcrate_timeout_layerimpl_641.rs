// Generated macro for impl_641 (impl)
macro_rules! Depcrate_timeout_layerimpl_641 {
() => {
// Module: crate::timeout::layer
// Provides: {"impl_641"}
// Dependencies: {}
impl < S > Layer < S > for TimeoutLayer { type Service = Timeout < S > ; fn layer (& self , service : S) -> Self :: Service { Timeout :: new (service , self . timeout) } }
};
}
