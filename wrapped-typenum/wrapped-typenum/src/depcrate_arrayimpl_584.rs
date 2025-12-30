// Generated macro for impl_584 (impl)
macro_rules! Depcrate_arrayimpl_584 {
() => {
// Module: crate::array
// Provides: {"impl_584"}
// Dependencies: {}
impl < V , A > FoldMul for TArr < V , A > where A : FoldMul , FoldProd < A > : Mul < V > , { type Output = Prod < FoldProd < A > , V > ; }
};
}
