// Generated macro for impl_420 (impl)
macro_rules! Depcrate_uintimpl_420 {
() => {
// Module: crate::uint
// Provides: {"impl_420"}
// Dependencies: {}
# [doc = " Shifting left a `UInt` by a one bit: `UInt<U, B> << B1 = UInt<UInt<U, B>, B0>`"] impl < U : Unsigned , B : Bit > Shl < B1 > for UInt < U , B > { type Output = UInt < UInt < U , B > , B0 > ; # [inline] fn shl (self , _ : B1) -> Self :: Output { UInt :: new () } }
};
}
