// Generated macro for impl_612 (impl)
macro_rules! Depcrate_streamimpl_612 {
() => {
// Module: crate::stream
// Provides: {"impl_612"}
// Dependencies: {}
impl < T : PartialOrd , S > PartialOrd for Checkpoint < T , S > { # [inline (always)] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . inner . partial_cmp (& other . inner) } }
};
}
