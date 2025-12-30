// Generated macro for impl_505 (impl)
macro_rules! Depcrate_uintimpl_505 {
() => {
// Module: crate::uint
// Provides: {"impl_505"}
// Dependencies: {}
impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned , Br : Bit > Div < UInt < Ur , Br > > for UInt < Ul , Bl > where UInt < Ul , Bl > : Len , Length < UInt < Ul , Bl > > : Sub < B1 > , () : PrivateDiv < UInt < Ul , Bl > , UInt < Ur , Br > , U0 , U0 , Sub1 < Length < UInt < Ul , Bl > > > > , { type Output = PrivateDivQuot < UInt < Ul , Bl > , UInt < Ur , Br > , U0 , U0 , Sub1 < Length < UInt < Ul , Bl > > > > ; # [inline] fn div (self , rhs : UInt < Ur , Br >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] () . private_div_quotient (self , rhs , U0 :: new () , U0 :: new () , self . len () - B1) } }
};
}
