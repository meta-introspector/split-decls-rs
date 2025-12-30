// Generated macro for impl_278 (impl)
macro_rules! Depcrate_stream_bytesimpl_278 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a > Offset < < & 'a Bytes as Stream > :: Checkpoint > for & 'a Bytes { # [inline (always)] fn offset_from (& self , other : & < & 'a Bytes as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
