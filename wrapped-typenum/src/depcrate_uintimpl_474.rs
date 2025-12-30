// Generated macro for impl_474 (impl)
macro_rules! Depcrate_uintimpl_474 {
() => {
// Module: crate::uint
// Provides: {"impl_474"}
// Dependencies: {}
impl < Y : Unsigned , X : Unsigned > PrivatePow < Y , U1 > for X where X : Mul < Y > , { type Output = Prod < X , Y > ; # [inline] fn private_pow (self , y : Y , _ : U1) -> Self :: Output { self * y } }
};
}
