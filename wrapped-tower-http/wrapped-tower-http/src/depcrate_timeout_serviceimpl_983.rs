// Generated macro for impl_983 (impl)
macro_rules! Depcrate_timeout_serviceimpl_983 {
() => {
// Module: crate::timeout::service
// Provides: {"impl_983"}
// Dependencies: {}
impl < S > Layer < S > for RequestBodyTimeoutLayer { type Service = RequestBodyTimeout < S > ; fn layer (& self , inner : S) -> Self :: Service { RequestBodyTimeout :: new (inner , self . timeout) } }
};
}
