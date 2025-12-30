// Generated macro for impl_438 (impl)
macro_rules! Depcrate_uintimpl_438 {
() => {
// Module: crate::uint
// Provides: {"impl_438"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> * UInt<Ur, B> = UInt<(Ul * UInt<Ur, B>), B0> + UInt<Ur, B>`"] impl < Ul : Unsigned , B : Bit , Ur : Unsigned > Mul < UInt < Ur , B > > for UInt < Ul , B1 > where Ul : Mul < UInt < Ur , B > > , UInt < Prod < Ul , UInt < Ur , B > > , B0 > : Add < UInt < Ur , B > > , { type Output = Sum < UInt < Prod < Ul , UInt < Ur , B > > , B0 > , UInt < Ur , B > > ; # [inline] fn mul (self , rhs : UInt < Ur , B >) -> Self :: Output { UInt { msb : self . msb * rhs , lsb : B0 , } + rhs } }
};
}
