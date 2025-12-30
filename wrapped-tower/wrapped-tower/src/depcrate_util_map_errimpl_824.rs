// Generated macro for impl_824 (impl)
macro_rules! Depcrate_util_map_errimpl_824 {
() => {
// Module: crate::util::map_err
// Provides: {"impl_824"}
// Dependencies: {}
impl < S , F > Layer < S > for MapErrLayer < F > where F : Clone , { type Service = MapErr < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapErr { f : self . f . clone () , inner , } } }
};
}
