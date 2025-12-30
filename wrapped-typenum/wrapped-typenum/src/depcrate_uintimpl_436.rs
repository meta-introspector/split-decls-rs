// Generated macro for impl_436 (impl)
macro_rules! Depcrate_uintimpl_436 {
() => {
// Module: crate::uint
// Provides: {"impl_436"}
// Dependencies: {}
# [doc = " Shifting left `UInt` by `UTerm`: `UInt<U, B> << UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Shl < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shl (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
};
}
