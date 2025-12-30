// Generated macro for impl_421 (impl)
macro_rules! Depcrate_uintimpl_421 {
() => {
// Module: crate::uint
// Provides: {"impl_421"}
// Dependencies: {}
# [doc = " Shifting left `UInt` by `UTerm`: `UInt<U, B> << UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Shl < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shl (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
};
}
