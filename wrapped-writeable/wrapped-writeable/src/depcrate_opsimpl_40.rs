// Generated macro for impl_40 (impl)
macro_rules! Depcrate_opsimpl_40 {
() => {
// Module: crate::ops
// Provides: {"impl_40"}
// Dependencies: {}
impl core :: iter :: Sum < LengthHint > for LengthHint { fn sum < I > (iter : I) -> Self where I : Iterator < Item = LengthHint > , { iter . fold (LengthHint :: exact (0) , core :: ops :: Add :: add) } }
};
}
