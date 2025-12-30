// Generated macro for impl_561 (impl)
macro_rules! Depcrate_arrayimpl_561 {
() => {
// Module: crate::array
// Provides: {"impl_561"}
// Dependencies: {}
impl < V , A > FoldAdd for TArr < V , A > where A : FoldAdd , FoldSum < A > : Add < V > , { type Output = Sum < FoldSum < A > , V > ; }
};
}
