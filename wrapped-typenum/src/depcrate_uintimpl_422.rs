// Generated macro for impl_422 (impl)
macro_rules! Depcrate_uintimpl_422 {
() => {
// Module: crate::uint
// Provides: {"impl_422"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> | UInt<Ur, B0> = UInt<Ul | Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > BitOr < UInt < Ur , B0 > > for UInt < Ul , B1 > where Ul : BitOr < Ur > , { type Output = UInt < Or < Ul , Ur > , B1 > ; # [inline] fn bitor (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . bitor (rhs . msb) , lsb : self . lsb . bitor (rhs . lsb) , } } }
};
}
