// Generated macro for impl_133 (impl)
macro_rules! Depcrate_intimpl_133 {
() => {
// Module: crate::int
// Provides: {"impl_133"}
// Dependencies: {}
impl < U1 , U2 > Gcd < PInt < U2 > > for PInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
};
}
