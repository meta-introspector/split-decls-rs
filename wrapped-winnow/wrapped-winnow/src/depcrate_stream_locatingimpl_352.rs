// Generated macro for impl_352 (impl)
macro_rules! Depcrate_stream_locatingimpl_352 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_352"}
// Dependencies: {}
impl < I > UpdateSlice for LocatingSlice < I > where I : UpdateSlice , { # [inline (always)] fn update_slice (mut self , inner : Self :: Slice) -> Self { self . input = I :: update_slice (self . input , inner) ; self } }
};
}
