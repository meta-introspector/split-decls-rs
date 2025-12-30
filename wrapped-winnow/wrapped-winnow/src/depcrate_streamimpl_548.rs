// Generated macro for impl_548 (impl)
macro_rules! Depcrate_streamimpl_548 {
() => {
// Module: crate::stream
// Provides: {"impl_548"}
// Dependencies: {}
impl < 'a , T > Offset < < & 'a [T] as Stream > :: Checkpoint > for & 'a [T] where T : Clone + core :: fmt :: Debug , { # [inline (always)] fn offset_from (& self , other : & < & 'a [T] as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
