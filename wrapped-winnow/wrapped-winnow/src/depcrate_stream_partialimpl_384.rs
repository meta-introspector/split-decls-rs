// Generated macro for impl_384 (impl)
macro_rules! Depcrate_stream_partialimpl_384 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_384"}
// Dependencies: {}
impl < I > UpdateSlice for Partial < I > where I : UpdateSlice , { # [inline (always)] fn update_slice (self , inner : Self :: Slice) -> Self { Partial { input : I :: update_slice (self . input , inner) , partial : self . partial , } } }
};
}
