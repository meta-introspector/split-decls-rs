// Generated macro for impl_416 (impl)
macro_rules! Depcrate_uintimpl_416 {
() => {
// Module: crate::uint
// Provides: {"impl_416"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> ^ UInt<Ur, B1> = UInt<Ul ^ Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateXor < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : PrivateXor < Ur > , { type Output = UInt < PrivateXorOut < Ul , Ur > , B0 > ; # [inline] fn private_xor (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_xor (rhs . msb) , lsb : B0 , } } }
};
}
