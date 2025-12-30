// Generated macro for impl_1266 (impl)
macro_rules! Depcrate_validate_requestimpl_1266 {
() => {
// Module: crate::validate_request
// Provides: {"impl_1266"}
// Dependencies: {}
impl < S , T > Layer < S > for ValidateRequestHeaderLayer < T > where T : Clone , { type Service = ValidateRequestHeader < S , T > ; fn layer (& self , inner : S) -> Self :: Service { ValidateRequestHeader :: new (inner , self . validate . clone ()) } }
};
}
