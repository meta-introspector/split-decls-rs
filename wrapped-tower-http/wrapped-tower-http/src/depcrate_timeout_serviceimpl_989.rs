// Generated macro for impl_989 (impl)
macro_rules! Depcrate_timeout_serviceimpl_989 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_989"}
// Dependencies: {}
impl < S > Layer < S > for ResponseBodyTimeoutLayer { type Service = ResponseBodyTimeout < S > ; fn layer (& self , inner : S) -> Self :: Service { ResponseBodyTimeout :: new (inner , self . timeout) } }
};
}
