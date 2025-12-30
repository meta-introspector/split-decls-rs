// Generated macro for impl_468 (impl)
macro_rules! Depcrate_uintimpl_468 {
() => {
// Module: crate::uint
// Provides: {"impl_468"}
// Dependencies: {}
# [doc = " gcd(x, y) = gcd(x, y/2) if x odd and y even"] impl < Xp , Yp > Gcd < Even < Yp > > for Odd < Xp > where Odd < Xp > : Gcd < Yp > , Even < Yp > : NonZero , { type Output = Gcf < Odd < Xp > , Yp > ; }
};
}
