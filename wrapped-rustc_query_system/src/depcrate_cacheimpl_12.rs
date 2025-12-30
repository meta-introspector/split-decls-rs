// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cacheimpl_12 {
() => {
// Module: crate::cache
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : Clone > WithDepNode < T > { pub fn new (dep_node : DepNodeIndex , cached_value : T) -> Self { WithDepNode { dep_node , cached_value } } pub fn get < Tcx : DepContext > (& self , tcx : Tcx) -> T { tcx . dep_graph () . read_index (self . dep_node) ; self . cached_value . clone () } }
};
}
