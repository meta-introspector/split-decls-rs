// Generated macro for impl_347 (impl)
macro_rules! Depcrate_stream_locatingimpl_347 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_347"}
// Dependencies: {}
impl < I > Offset < < LocatingSlice < I > as Stream > :: Checkpoint > for LocatingSlice < I > where I : Stream , { # [inline (always)] fn offset_from (& self , other : & < LocatingSlice < I > as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
