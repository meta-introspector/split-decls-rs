// Generated macro for impl_385 (impl)
macro_rules! Depcrate_uintimpl_385 {
() => {
// Module: crate::uint
// Provides: {"impl_385"}
// Dependencies: {}
# [doc = " `UInt<U, B1> - B1 = UInt<U, B0>`"] impl < U : Unsigned , B : Bit > Sub < B1 > for UInt < UInt < U , B > , B1 > { type Output = UInt < UInt < U , B > , B0 > ; # [inline] fn sub (self , _ : B1) -> Self :: Output { UInt :: new () } }
};
}
