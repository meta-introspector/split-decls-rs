// Generated macro for impl_457 (impl)
macro_rules! Depcrate_stream_statefulimpl_457 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_457"}
// Dependencies: {}
impl < I , S > Offset < < Stateful < I , S > as Stream > :: Checkpoint > for Stateful < I , S > where I : Stream , S : core :: fmt :: Debug , { # [inline (always)] fn offset_from (& self , other : & < Stateful < I , S > as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
