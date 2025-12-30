// Generated macro for impl_577 (impl)
macro_rules! Depcrate_arrayimpl_577 {
() => {
// Module: crate::array
// Provides: {"impl_577"}
// Dependencies: {}
impl < V , A , Rhs > Div < Rhs > for TArr < V , A > where V : Div < Rhs > , A : Div < Rhs > , Rhs : Copy , { type Output = TArr < Quot < V , Rhs > , Quot < A , Rhs > > ; # [inline] fn div (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first / rhs , rest : self . rest / rhs , } } }
};
}
