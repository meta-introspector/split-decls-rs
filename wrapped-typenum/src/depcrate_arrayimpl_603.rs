// Generated macro for impl_603 (impl)
macro_rules! Depcrate_arrayimpl_603 {
() => {
// Module: crate::array
// Provides: {"impl_603"}
// Dependencies: {}
impl < V , A , Rhs > Rem < Rhs > for TArr < V , A > where V : Rem < Rhs > , A : Rem < Rhs > , Rhs : Copy , { type Output = TArr < Mod < V , Rhs > , Mod < A , Rhs > > ; # [inline] fn rem (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first % rhs , rest : self . rest % rhs , } } }
};
}
