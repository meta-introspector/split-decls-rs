// Generated macro for impl_573 (impl)
macro_rules! Depcrate_arrayimpl_573 {
() => {
// Module: crate::array
// Provides: {"impl_573"}
// Dependencies: {}
impl < V , A > Mul < TArr < V , A > > for Z0 where Z0 : Mul < A > , { type Output = TArr < Z0 , Prod < Z0 , A > > ; # [inline] fn mul (self , rhs : TArr < V , A >) -> Self :: Output { TArr { first : Z0 , rest : self * rhs . rest , } } }
};
}
