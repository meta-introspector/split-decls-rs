// Generated macro for impl_439 (impl)
macro_rules! Depcrate_map_response_bodyimpl_439 {
() => {
// Module: crate::map_response_body
// Provides: {"impl_439"}
// Dependencies: {}
impl < S , F > Layer < S > for MapResponseBodyLayer < F > where F : Clone , { type Service = MapResponseBody < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapResponseBody :: new (inner , self . f . clone ()) } }
};
}
