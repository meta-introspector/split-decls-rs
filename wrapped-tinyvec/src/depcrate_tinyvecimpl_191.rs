// Generated macro for impl_191 (impl)
macro_rules! Depcrate_tinyvecimpl_191 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_191"}
// Dependencies: {}
impl < A : Array > PartialOrd for TinyVec < A > where A :: Item : PartialOrd , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_slice ()) } }
};
}
