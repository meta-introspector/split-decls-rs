// Generated macro for impl_875 (impl)
macro_rules! Depcrate_util_map_futureimpl_875 {
() => {
// Module: crate::util::map_future
// Provides: {"impl_875"}
// Dependencies: {}
impl < S , F > Layer < S > for MapFutureLayer < F > where F : Clone , { type Service = MapFuture < S , F > ; fn layer (& self , inner : S) -> Self :: Service { MapFuture :: new (inner , self . f . clone ()) } }
};
}
