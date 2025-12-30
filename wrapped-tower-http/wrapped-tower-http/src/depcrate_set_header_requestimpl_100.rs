// Generated macro for impl_100 (impl)
macro_rules! Depcrate_set_header_requestimpl_100 {
() => {
// Module: crate::set_header::request
// Provides: {"impl_100"}
// Dependencies: {}
impl < S , M > Layer < S > for SetRequestHeaderLayer < M > where M : Clone , { type Service = SetRequestHeader < S , M > ; fn layer (& self , inner : S) -> Self :: Service { SetRequestHeader { inner , header_name : self . header_name . clone () , make : self . make . clone () , mode : self . mode , } } }
};
}
