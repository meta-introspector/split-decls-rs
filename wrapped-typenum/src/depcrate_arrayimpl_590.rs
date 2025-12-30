// Generated macro for impl_590 (impl)
macro_rules! Depcrate_arrayimpl_590 {
() => {
// Module: crate::array
// Provides: {"impl_590"}
// Dependencies: {}
impl < V , A , Rhs > Mul < Rhs > for TArr < V , A > where V : Mul < Rhs > , A : Mul < Rhs > , Rhs : Copy , { type Output = TArr < Prod < V , Rhs > , Prod < A , Rhs > > ; # [inline] fn mul (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first * rhs , rest : self . rest * rhs , } } }
};
}
