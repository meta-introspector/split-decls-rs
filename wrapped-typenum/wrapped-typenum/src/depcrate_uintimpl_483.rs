// Generated macro for impl_483 (impl)
macro_rules! Depcrate_uintimpl_483 {
() => {
// Module: crate::uint
// Provides: {"impl_483"}
// Dependencies: {}
# [doc = " gcd(x, y) = gcd(x, y/2) if x odd and y even"] impl < Xp , Yp > Gcd < Even < Yp > > for Odd < Xp > where Odd < Xp > : Gcd < Yp > , Even < Yp > : NonZero , { type Output = Gcf < Odd < Xp > , Yp > ; }
};
}
