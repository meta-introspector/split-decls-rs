// Generated macro for impl_563 (impl)
macro_rules! Depcrate_arrayimpl_563 {
() => {
// Module: crate::array
// Provides: {"impl_563"}
// Dependencies: {}
impl < V , A > FoldMul for TArr < V , A > where A : FoldMul , FoldProd < A > : Mul < V > , { type Output = Prod < FoldProd < A > , V > ; }
};
}
