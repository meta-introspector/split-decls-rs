// Generated macro for impl_482 (impl)
macro_rules! Depcrate_uintimpl_482 {
() => {
// Module: crate::uint
// Provides: {"impl_482"}
// Dependencies: {}
# [doc = " gcd(x, y) = 2*gcd(x/2, y/2) if both x and y even"] impl < Xp , Yp > Gcd < Even < Yp > > for Even < Xp > where Xp : Gcd < Yp > , Even < Xp > : NonZero , Even < Yp > : NonZero , { type Output = UInt < Gcf < Xp , Yp > , B0 > ; }
};
}
