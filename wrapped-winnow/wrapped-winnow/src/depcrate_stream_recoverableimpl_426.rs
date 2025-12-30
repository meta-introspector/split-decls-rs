// Generated macro for impl_426 (impl)
macro_rules! Depcrate_stream_recoverableimpl_426 {
() => {
// Module: crate::stream::recoverable
// Provides: {"impl_426"}
// Dependencies: {}
impl < I , E > Offset < < Recoverable < I , E > as Stream > :: Checkpoint > for Recoverable < I , E > where I : Stream , E : core :: fmt :: Debug , { # [inline (always)] fn offset_from (& self , other : & < Recoverable < I , E > as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
