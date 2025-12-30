// Generated macro for impl_900 (impl)
macro_rules! Depcrate_request_idimpl_900 {
() => {
// Module: crate::request_id
// Provides: {"impl_900"}
// Dependencies: {}
impl < S > Layer < S > for PropagateRequestIdLayer { type Service = PropagateRequestId < S > ; fn layer (& self , inner : S) -> Self :: Service { PropagateRequestId :: new (inner , self . header_name . clone ()) } }
};
}
