// Generated macro for impl_457 (impl)
macro_rules! Depcrate_uintimpl_457 {
() => {
// Module: crate::uint
// Provides: {"impl_457"}
// Dependencies: {}
# [doc = " X^N"] impl < X : Unsigned , N : Unsigned > Pow < N > for X where X : PrivatePow < U1 , N > , { type Output = PrivatePowOut < X , U1 , N > ; # [inline] fn powi (self , n : N) -> Self :: Output { self . private_pow (U1 :: new () , n) } }
};
}
