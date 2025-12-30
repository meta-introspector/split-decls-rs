// Generated macro for impl_62 (impl)
macro_rules! Depcrate_arrayvecimpl_62 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_62"}
// Dependencies: {}
impl < A : Array > PartialOrd for ArrayVec < A > where A :: Item : PartialOrd , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
};
}
