// Generated macro for impl_62 (impl)
macro_rules! Depcrate_dep_graph_edgesimpl_62 {
() => {
// Module: crate::dep_graph::edges
// Provides: {"impl_62"}
// Dependencies: {}
impl Extend < DepNodeIndex > for EdgesVec { # [inline] fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = DepNodeIndex > , { for elem in iter { self . push (elem) ; } } }
};
}
