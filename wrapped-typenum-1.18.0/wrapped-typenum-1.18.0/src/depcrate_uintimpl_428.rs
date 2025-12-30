// Generated macro for impl_428 (impl)
macro_rules! Depcrate_uintimpl_428 {
() => {
// Module: crate::uint
// Provides: {"impl_428"}
// Dependencies: {}
# [doc = " Shifting right any unsigned by a zero bit: `U >> B0 = U`"] impl < U : Unsigned , B : Bit > Shr < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shr (self , _ : B0) -> Self :: Output { UInt :: new () } }
};
}
