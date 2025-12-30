// Generated macro for impl_192 (impl)
macro_rules! Depcrate_tinyvecimpl_192 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_192"}
// Dependencies: {}
impl < A : Array > Ord for TinyVec < A > where A :: Item : Ord , { # [inline] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
};
}
