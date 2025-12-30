// Generated macro for impl_595 (impl)
macro_rules! Depcrate_arrayimpl_595 {
() => {
// Module: crate::array
// Provides: {"impl_595"}
// Dependencies: {}
impl < V , A , U > Mul < TArr < V , A > > for PInt < U > where U : Unsigned + NonZero , PInt < U > : Mul < A > + Mul < V > , { type Output = TArr < Prod < PInt < U > , V > , Prod < PInt < U > , A > > ; # [inline] fn mul (self , rhs : TArr < V , A >) -> Self :: Output { TArr { first : self * rhs . first , rest : self * rhs . rest , } } }
};
}
