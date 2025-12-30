// Generated macro for impl_376 (impl)
macro_rules! Depcrate_uintimpl_376 {
() => {
// Module: crate::uint
// Provides: {"impl_376"}
// Dependencies: {}
# [doc = " `UInt<U, B1> + B1 = UInt<U + B1, B0>`"] impl < U : Unsigned > Add < B1 > for UInt < U , B1 > where U : Add < B1 > , Add1 < U > : Unsigned , { type Output = UInt < Add1 < U > , B0 > ; # [inline] fn add (self , _ : B1) -> Self :: Output { UInt :: new () } }
};
}
