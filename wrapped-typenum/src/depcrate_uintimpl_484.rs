// Generated macro for impl_484 (impl)
macro_rules! Depcrate_uintimpl_484 {
() => {
// Module: crate::uint
// Provides: {"impl_484"}
// Dependencies: {}
# [doc = " gcd(x, y) = gcd(x/2, y) if x even and y odd"] impl < Xp , Yp > Gcd < Odd < Yp > > for Even < Xp > where Xp : Gcd < Odd < Yp > > , Even < Xp > : NonZero , { type Output = Gcf < Xp , Odd < Yp > > ; }
};
}
