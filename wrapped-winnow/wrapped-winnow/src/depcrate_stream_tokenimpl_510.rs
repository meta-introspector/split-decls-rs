// Generated macro for impl_510 (impl)
macro_rules! Depcrate_stream_tokenimpl_510 {
() => {
// Module: crate::stream::token
// Provides: {"impl_510"}
// Dependencies: {}
impl < T > UpdateSlice for TokenSlice < '_ , T > where T : core :: fmt :: Debug + Clone , { # [inline (always)] fn update_slice (mut self , inner : Self :: Slice) -> Self { self . input = < & [T] as UpdateSlice > :: update_slice (self . input , inner) ; self } }
};
}
