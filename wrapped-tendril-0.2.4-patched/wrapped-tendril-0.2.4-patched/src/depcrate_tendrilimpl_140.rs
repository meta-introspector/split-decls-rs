// Generated macro for impl_140 (impl)
macro_rules! Depcrate_tendrilimpl_140 {
() => {
// Module: crate::tendril
// Provides: {"impl_140"}
// Dependencies: {}
impl < F , A > PartialOrd for Tendril < F , A > where F : fmt :: SliceFormat , < F as fmt :: SliceFormat > :: Slice : PartialOrd , A : Atomicity , { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } }
};
}
