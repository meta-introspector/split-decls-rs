// Generated macro for impl_141 (impl)
macro_rules! Depcrate_tendrilimpl_141 {
() => {
// Module: crate::tendril
// Provides: {"impl_141"}
// Dependencies: {}
impl < F , A > Ord for Tendril < F , A > where F : fmt :: SliceFormat , < F as fmt :: SliceFormat > :: Slice : Ord , A : Atomicity , { # [inline] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
