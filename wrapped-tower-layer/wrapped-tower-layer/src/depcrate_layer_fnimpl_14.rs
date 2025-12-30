// Generated macro for impl_14 (impl)
macro_rules! Depcrate_layer_fnimpl_14 {
() => {
// Module: crate::layer_fn
// Provides: {"impl_14"}
// Dependencies: {}
impl < F , S , Out > Layer < S > for LayerFn < F > where F : Fn (S) -> Out , { type Service = Out ; fn layer (& self , inner : S) -> Self :: Service { (self . f) (inner) } }
};
}
