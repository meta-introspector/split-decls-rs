// Generated macro for impl_552 (impl)
macro_rules! Depcrate_streamimpl_552 {
() => {
// Module: crate::stream
// Provides: {"impl_552"}
// Dependencies: {}
impl < I > Offset < < (I , usize) as Stream > :: Checkpoint > for (I , usize) where I : Stream < Token = u8 > + Clone , { # [inline (always)] fn offset_from (& self , other : & < (I , usize) as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
