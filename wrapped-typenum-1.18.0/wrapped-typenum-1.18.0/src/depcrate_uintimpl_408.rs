// Generated macro for impl_408 (impl)
macro_rules! Depcrate_uintimpl_408 {
() => {
// Module: crate::uint
// Provides: {"impl_408"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > BitOr < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : BitOr < Ur > , { type Output = UInt < Or < Ul , Ur > , B1 > ; # [inline] fn bitor (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . bitor (rhs . msb) , lsb : self . lsb . bitor (rhs . lsb) , } } }
};
}
