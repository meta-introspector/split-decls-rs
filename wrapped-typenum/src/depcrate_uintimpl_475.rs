// Generated macro for impl_475 (impl)
macro_rules! Depcrate_uintimpl_475 {
() => {
// Module: crate::uint
// Provides: {"impl_475"}
// Dependencies: {}
# [doc = " N is even"] impl < Y : Unsigned , U : Unsigned , B : Bit , X : Unsigned > PrivatePow < Y , UInt < UInt < U , B > , B0 > > for X where X : Mul , Square < X > : PrivatePow < Y , UInt < U , B > > , { type Output = PrivatePowOut < Square < X > , Y , UInt < U , B > > ; # [inline] fn private_pow (self , y : Y , n : UInt < UInt < U , B > , B0 >) -> Self :: Output { (self * self) . private_pow (y , n . msb) } }
};
}
