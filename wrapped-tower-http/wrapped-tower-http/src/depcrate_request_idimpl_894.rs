// Generated macro for impl_894 (impl)
macro_rules! Depcrate_request_idimpl_894 {
() => {
// Module: crate::request_id
// Provides: {"impl_894"}
// Dependencies: {}
impl < S , M > Layer < S > for SetRequestIdLayer < M > where M : Clone + MakeRequestId , { type Service = SetRequestId < S , M > ; fn layer (& self , inner : S) -> Self :: Service { SetRequestId :: new (inner , self . header_name . clone () , self . make_request_id . clone () ,) } }
};
}
