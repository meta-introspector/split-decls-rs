// Generated macro for impl_582 (impl)
macro_rules! Depcrate_arrayimpl_582 {
() => {
// Module: crate::array
// Provides: {"impl_582"}
// Dependencies: {}
impl < V , A > FoldAdd for TArr < V , A > where A : FoldAdd , FoldSum < A > : Add < V > , { type Output = Sum < FoldSum < A > , V > ; }
};
}
