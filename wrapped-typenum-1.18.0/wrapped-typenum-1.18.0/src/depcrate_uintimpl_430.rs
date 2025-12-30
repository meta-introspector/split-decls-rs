// Generated macro for impl_430 (impl)
macro_rules! Depcrate_uintimpl_430 {
() => {
// Module: crate::uint
// Provides: {"impl_430"}
// Dependencies: {}
# [doc = " Shifting right `UInt` by `UInt`: `UInt(U, B) >> Y` = `U >> (Y - 1)`"] impl < U : Unsigned , B : Bit , Ur : Unsigned , Br : Bit > Shr < UInt < Ur , Br > > for UInt < U , B > where UInt < Ur , Br > : Sub < B1 > , U : Shr < Sub1 < UInt < Ur , Br > > > , { type Output = Shright < U , Sub1 < UInt < Ur , Br > > > ; # [inline] fn shr (self , rhs : UInt < Ur , Br >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] self . msb . shr (rhs - B1) } }
};
}
