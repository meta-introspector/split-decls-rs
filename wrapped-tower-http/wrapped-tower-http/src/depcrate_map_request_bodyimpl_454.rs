// Generated macro for impl_454 (impl)
macro_rules! Depcrate_map_request_bodyimpl_454 {
() => {
// Module: crate::map_request_body
// Provides: {"impl_454"}
// Dependencies: {}
impl < S , F > Layer < S > for MapRequestBodyLayer < F > where F : Clone , { type Service = MapRequestBody < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapRequestBody :: new (inner , self . f . clone ()) } }
};
}
