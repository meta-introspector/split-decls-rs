// Generated macro for impl_462 (impl)
macro_rules! Depcrate_stream_statefulimpl_462 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_462"}
// Dependencies: {}
impl < I , S > UpdateSlice for Stateful < I , S > where I : UpdateSlice , S : Clone + core :: fmt :: Debug , { # [inline (always)] fn update_slice (mut self , inner : Self :: Slice) -> Self { self . input = I :: update_slice (self . input , inner) ; self } }
};
}
