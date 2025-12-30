// Generated macro for impl_440 (impl)
macro_rules! Depcrate_uintimpl_440 {
() => {
// Module: crate::uint
// Provides: {"impl_440"}
// Dependencies: {}
# [doc = " Nonzero > Zero"] impl < U : Unsigned , B : Bit > Cmp < UTerm > for UInt < U , B > { type Output = Greater ; # [inline] fn compare < IM : InternalMarker > (& self , _ : & UTerm) -> Self :: Output { Greater } }
};
}
