// Generated macro for impl_485 (impl)
macro_rules! Depcrate_uintimpl_485 {
() => {
// Module: crate::uint
// Provides: {"impl_485"}
// Dependencies: {}
# [doc = " gcd(x, y) = gcd([max(x, y) - min(x, y)], min(x, y)) if both x and y odd"] # [doc = ""] # [doc = " This will immediately invoke the case for x even and y odd because the difference of two odd"] # [doc = " numbers is an even number."] impl < Xp , Yp > Gcd < Odd < Yp > > for Odd < Xp > where Odd < Xp > : Max < Odd < Yp > > + Min < Odd < Yp > > , Odd < Yp > : Max < Odd < Xp > > + Min < Odd < Xp > > , Maximum < Odd < Xp > , Odd < Yp > > : Sub < Minimum < Odd < Xp > , Odd < Yp > > > , Diff < Maximum < Odd < Xp > , Odd < Yp > > , Minimum < Odd < Xp > , Odd < Yp > > > : Gcd < Minimum < Odd < Xp > , Odd < Yp > > > , { type Output = Gcf < Diff < Maximum < Odd < Xp > , Odd < Yp > > , Minimum < Odd < Xp > , Odd < Yp > > > , Minimum < Odd < Xp > , Odd < Yp > > > ; }
};
}
