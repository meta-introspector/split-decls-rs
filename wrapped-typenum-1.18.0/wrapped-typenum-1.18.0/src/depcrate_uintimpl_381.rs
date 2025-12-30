// Generated macro for impl_381 (impl)
macro_rules! Depcrate_uintimpl_381 {
() => {
// Module: crate::uint
// Provides: {"impl_381"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> + UInt<Ur, B0> = UInt<Ul + Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B0 > > for UInt < Ul , B1 > where Ul : Add < Ur > , { type Output = UInt < Sum < Ul , Ur > , B1 > ; # [inline] fn add (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb , lsb : B1 , } } }
};
}
