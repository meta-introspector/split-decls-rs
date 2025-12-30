// Generated macro for impl_975 (impl)
macro_rules! Depcrate_timeout_serviceimpl_975 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_975"}
// Dependencies: {}
impl < S > Layer < S > for TimeoutLayer { type Service = Timeout < S > ; fn layer (& self , inner : S) -> Self :: Service { Timeout :: with_status_code (inner , self . status_code , self . timeout) } }
};
}
