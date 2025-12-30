// Generated macro for impl_836 (impl)
macro_rules! Depcrate_util_map_requestimpl_836 {
() => {
// Module: crate::util::map_request
// Provides: {"impl_836"}
// Dependencies: {}
impl < S , F > Layer < S > for MapRequestLayer < F > where F : Clone , { type Service = MapRequest < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapRequest { f : self . f . clone () , inner , } } }
};
}
