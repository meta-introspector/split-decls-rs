// Generated macro for impl_379 (impl)
macro_rules! Depcrate_stream_partialimpl_379 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_379"}
// Dependencies: {}
impl < I > Offset < < Partial < I > as Stream > :: Checkpoint > for Partial < I > where I : Stream , { # [inline (always)] fn offset_from (& self , other : & < Partial < I > as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
