// Generated macro for impl_420 (impl)
macro_rules! Depcrate_uintimpl_420 {
() => {
// Module: crate::uint
// Provides: {"impl_420"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> | UInt<Ur, B0> = UInt<Ul | Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > BitOr < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : BitOr < Ur > , { type Output = UInt < < Ul as BitOr < Ur > > :: Output , B0 > ; # [inline] fn bitor (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . bitor (rhs . msb) , lsb : B0 , } } }
};
}
