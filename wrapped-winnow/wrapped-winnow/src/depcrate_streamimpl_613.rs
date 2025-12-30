// Generated macro for impl_613 (impl)
macro_rules! Depcrate_streamimpl_613 {
() => {
// Module: crate::stream
// Provides: {"impl_613"}
// Dependencies: {}
impl < T : Ord , S > Ord for Checkpoint < T , S > { # [inline (always)] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . inner . cmp (& other . inner) } }
};
}
