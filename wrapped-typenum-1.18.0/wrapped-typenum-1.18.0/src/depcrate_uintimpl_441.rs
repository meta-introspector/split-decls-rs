// Generated macro for impl_441 (impl)
macro_rules! Depcrate_uintimpl_441 {
() => {
// Module: crate::uint
// Provides: {"impl_441"}
// Dependencies: {}
# [doc = " Zero < Nonzero"] impl < U : Unsigned , B : Bit > Cmp < UInt < U , B > > for UTerm { type Output = Less ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & UInt < U , B >) -> Self :: Output { Less } }
};
}
