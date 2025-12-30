// Generated macro for impl_215 (impl)
macro_rules! Depcrate_stream_bstrimpl_215 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'a > Offset < < & 'a BStr as Stream > :: Checkpoint > for & 'a BStr { # [inline (always)] fn offset_from (& self , other : & < & 'a BStr as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
