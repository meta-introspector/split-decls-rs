// Generated macro for impl_116 (impl)
macro_rules! Depcrate_set_header_responseimpl_116 {
() => {
// Module: crate::set_header::response
// Provides: {"impl_116"}
// Dependencies: {}
impl < S , M > Layer < S > for SetResponseHeaderLayer < M > where M : Clone , { type Service = SetResponseHeader < S , M > ; fn layer (& self , inner : S) -> Self :: Service { SetResponseHeader { inner , header_name : self . header_name . clone () , make : self . make . clone () , mode : self . mode , } } }
};
}
