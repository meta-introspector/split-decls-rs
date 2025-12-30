// Generated macro for impl_390 (impl)
macro_rules! Depcrate_uintimpl_390 {
() => {
// Module: crate::uint
// Provides: {"impl_390"}
// Dependencies: {}
# [doc = " `UInt<U, B0> + B1 = UInt<U + B1>`"] impl < U : Unsigned > Add < B1 > for UInt < U , B0 > { type Output = UInt < U , B1 > ; # [inline] fn add (self , _ : B1) -> Self :: Output { UInt :: new () } }
};
}
