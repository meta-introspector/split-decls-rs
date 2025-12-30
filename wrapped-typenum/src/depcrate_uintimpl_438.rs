// Generated macro for impl_438 (impl)
macro_rules! Depcrate_uintimpl_438 {
() => {
// Module: crate::uint
// Provides: {"impl_438"}
// Dependencies: {}
# [doc = " Shifting left `UInt` by `UInt`: `X << Y` = `UInt(X, B0) << (Y - 1)`"] impl < U : Unsigned , B : Bit , Ur : Unsigned , Br : Bit > Shl < UInt < Ur , Br > > for UInt < U , B > where UInt < Ur , Br > : Sub < B1 > , UInt < UInt < U , B > , B0 > : Shl < Sub1 < UInt < Ur , Br > > > , { type Output = Shleft < UInt < UInt < U , B > , B0 > , Sub1 < UInt < Ur , Br > > > ; # [inline] fn shl (self , rhs : UInt < Ur , Br >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] (UInt { msb : self , lsb : B0 }) . shl (rhs - B1) } }
};
}
