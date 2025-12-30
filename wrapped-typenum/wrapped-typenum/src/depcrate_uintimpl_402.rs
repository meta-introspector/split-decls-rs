// Generated macro for impl_402 (impl)
macro_rules! Depcrate_uintimpl_402 {
() => {
// Module: crate::uint
// Provides: {"impl_402"}
// Dependencies: {}
# [doc = " `UInt<U, B0> - B1 = UInt<U - B1, B1>`"] impl < U : Unsigned > Sub < B1 > for UInt < U , B0 > where U : Sub < B1 > , Sub1 < U > : Unsigned , { type Output = UInt < Sub1 < U > , B1 > ; # [inline] fn sub (self , _ : B1) -> Self :: Output { UInt :: new () } }
};
}
