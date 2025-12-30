// Generated macro for impl_135 (impl)
macro_rules! Depcrate_intimpl_135 {
() => {
// Module: crate::int
// Provides: {"impl_135"}
// Dependencies: {}
impl < U1 , U2 > Gcd < NInt < U2 > > for PInt < U1 > where U1 : Unsigned + NonZero + Gcd < U2 > , U2 : Unsigned + NonZero , Gcf < U1 , U2 > : Unsigned + NonZero , { type Output = PInt < Gcf < U1 , U2 > > ; }
};
}
