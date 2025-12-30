// Generated macro for impl_456 (impl)
macro_rules! Depcrate_stream_statefulimpl_456 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_456"}
// Dependencies: {}
impl < I , S > Offset for Stateful < I , S > where I : Stream , S : Clone + core :: fmt :: Debug , { # [inline (always)] fn offset_from (& self , start : & Self) -> usize { self . offset_from (& start . checkpoint ()) } }
};
}
