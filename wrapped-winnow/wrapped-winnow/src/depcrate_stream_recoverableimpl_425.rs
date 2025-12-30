// Generated macro for impl_425 (impl)
macro_rules! Depcrate_stream_recoverableimpl_425 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_425"}
// Dependencies: {}
impl < I , E > Offset for Recoverable < I , E > where I : Stream , E : core :: fmt :: Debug , { # [inline (always)] fn offset_from (& self , other : & Self) -> usize { self . offset_from (& other . checkpoint ()) } }
};
}
