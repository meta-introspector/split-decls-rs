// Generated macro for impl_61 (impl)
macro_rules! Depcrate_dep_graph_edgesimpl_61 {
() => {
// Module: crate::dep_graph::edges
// Provides: {"impl_61"}
// Dependencies: {}
impl FromIterator < DepNodeIndex > for EdgesVec { # [inline] fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = DepNodeIndex > , { let mut vec = EdgesVec :: new () ; for index in iter { vec . push (index) } vec } }
};
}
