// Generated macro for impl_550 (impl)
macro_rules! Depcrate_streamimpl_550 {
() => {
// Module: crate::stream
// Provides: {"impl_550"}
// Dependencies: {}
impl < 'a > Offset < < & 'a str as Stream > :: Checkpoint > for & 'a str { # [inline (always)] fn offset_from (& self , other : & < & 'a str as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
