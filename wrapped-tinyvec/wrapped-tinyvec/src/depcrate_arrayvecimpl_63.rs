// Generated macro for impl_63 (impl)
macro_rules! Depcrate_arrayvecimpl_63 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_63"}
// Dependencies: {}
impl < A : Array > Ord for ArrayVec < A > where A :: Item : Ord , { # [inline] fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
};
}
