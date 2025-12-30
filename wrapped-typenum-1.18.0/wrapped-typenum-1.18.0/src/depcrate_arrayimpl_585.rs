// Generated macro for impl_585 (impl)
macro_rules! Depcrate_arrayimpl_585 {
() => {
// Module: crate::array
// Provides: {"impl_585"}
// Dependencies: {}
impl < V , A > Neg for TArr < V , A > where V : Neg , A : Neg , { type Output = TArr < Negate < V > , Negate < A > > ; # [inline] fn neg (self) -> Self :: Output { TArr { first : - self . first , rest : - self . rest , } } }
};
}
