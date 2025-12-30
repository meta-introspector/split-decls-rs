// Generated macro for impl_459 (impl)
macro_rules! Depcrate_uintimpl_459 {
() => {
// Module: crate::uint
// Provides: {"impl_459"}
// Dependencies: {}
impl < Y : Unsigned , X : Unsigned > PrivatePow < Y , U1 > for X where X : Mul < Y > , { type Output = Prod < X , Y > ; # [inline] fn private_pow (self , y : Y , _ : U1) -> Self :: Output { self * y } }
};
}
