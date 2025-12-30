// Generated macro for impl_431 (impl)
macro_rules! Depcrate_stream_recoverableimpl_431 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_431"}
// Dependencies: {}
impl < I , E > UpdateSlice for Recoverable < I , E > where I : Stream , I : UpdateSlice , E : core :: fmt :: Debug , { # [inline (always)] fn update_slice (mut self , inner : Self :: Slice) -> Self { self . input = I :: update_slice (self . input , inner) ; self } }
};
}
