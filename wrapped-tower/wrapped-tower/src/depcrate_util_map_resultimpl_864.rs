// Generated macro for impl_864 (impl)
macro_rules! Depcrate_util_map_resultimpl_864 {
() => {
// Module: crate::util::map_result
// Provides: {"impl_864"}
// Dependencies: {}
impl < S , F > Layer < S > for MapResultLayer < F > where F : Clone , { type Service = MapResult < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapResult { f : self . f . clone () , inner , } } }
};
}
