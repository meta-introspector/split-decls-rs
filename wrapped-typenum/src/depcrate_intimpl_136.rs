// Generated macro for impl_136 (impl)
macro_rules! Depcrate_intimpl_136 {
() => {
// Module: crate::int
// Provides: {"impl_136"}
// Dependencies: {}
impl < U1 , U2 > Gcd < PInt < U2 > > for PInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
};
}
