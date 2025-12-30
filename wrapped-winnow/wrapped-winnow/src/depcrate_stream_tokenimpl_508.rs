// Generated macro for impl_508 (impl)
macro_rules! Depcrate_stream_tokenimpl_508 {
() => {
// Module: crate::stream::token
// Provides: {"impl_508"}
// Dependencies: {}
impl < T > Offset < < TokenSlice < '_ , T > as Stream > :: Checkpoint > for TokenSlice < '_ , T > where T : core :: fmt :: Debug + Clone , { # [inline (always)] fn offset_from (& self , other : & < TokenSlice < '_ , T > as Stream > :: Checkpoint) -> usize { self . checkpoint () . offset_from (other) } }
};
}
