// Generated macro for impl_600 (impl)
macro_rules! Depcrate_arrayimpl_600 {
() => {
// Module: crate::array
// Provides: {"impl_600"}
// Dependencies: {}
impl < V , A , Rhs > PartialDiv < Rhs > for TArr < V , A > where V : PartialDiv < Rhs > , A : PartialDiv < Rhs > , Rhs : Copy , { type Output = TArr < PartialQuot < V , Rhs > , PartialQuot < A , Rhs > > ; # [inline] fn partial_div (self , rhs : Rhs) -> Self :: Output { TArr { first : self . first . partial_div (rhs) , rest : self . rest . partial_div (rhs) , } } }
};
}
