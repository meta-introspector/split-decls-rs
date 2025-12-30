// Generated macro for impl_380 (impl)
macro_rules! Depcrate_uintimpl_380 {
() => {
// Module: crate::uint
// Provides: {"impl_380"}
// Dependencies: {}
# [doc = " Length of a bit is 1"] impl < U : Unsigned , B : Bit > Len for UInt < U , B > where U : Len , Length < U > : Add < B1 > , Add1 < Length < U > > : Unsigned , { type Output = Add1 < Length < U > > ; # [inline] fn len (& self) -> Self :: Output { self . msb . len () + B1 } }
};
}
