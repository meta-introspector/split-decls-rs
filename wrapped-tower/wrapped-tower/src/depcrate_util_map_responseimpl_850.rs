// Generated macro for impl_850 (impl)
macro_rules! Depcrate_util_map_responseimpl_850 {
() => {
// Module: crate::util::map_response
// Provides: {"impl_850"}
// Dependencies: {}
impl < S , F > Layer < S > for MapResponseLayer < F > where F : Clone , { type Service = MapResponse < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapResponse { f : self . f . clone () , inner , } } }
};
}
