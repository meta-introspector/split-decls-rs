// Generated macro for impl_507 (impl)
macro_rules! Depcrate_stream_tokenimpl_507 {
() => {
// Module: crate::stream::token
// Provides: {"impl_507"}
// Dependencies: {}
impl < T > Offset for TokenSlice < '_ , T > where T : core :: fmt :: Debug + Clone , { # [inline (always)] fn offset_from (& self , other : & Self) -> usize { self . offset_from (& other . checkpoint ()) } }
};
}
