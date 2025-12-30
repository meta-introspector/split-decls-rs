// Generated macro for impl_452 (impl)
macro_rules! Depcrate_uintimpl_452 {
() => {
// Module: crate::uint
// Provides: {"impl_452"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0>`"] impl < Ul : Unsigned , B : Bit , Ur : Unsigned > Mul < UInt < Ur , B > > for UInt < Ul , B0 > where Ul : Mul < UInt < Ur , B > > , { type Output = UInt < Prod < Ul , UInt < Ur , B > > , B0 > ; # [inline] fn mul (self , rhs : UInt < Ur , B >) -> Self :: Output { UInt { msb : self . msb * rhs , lsb : B0 , } } }
};
}
