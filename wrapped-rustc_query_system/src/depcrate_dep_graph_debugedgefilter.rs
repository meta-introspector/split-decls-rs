// Generated macro for EdgeFilter (struct)
macro_rules! Depcrate_dep_graph_debugEdgeFilter {
() => {
// Module: crate::dep_graph::debug
// Provides: {"EdgeFilter"}
// Dependencies: {}
# [doc = " A filter like `F -> G` where `F` and `G` are valid dep-node"] # [doc = " filters. This can be used to test the source/target independently."] pub struct EdgeFilter { pub source : DepNodeFilter , pub target : DepNodeFilter , pub index_to_node : Lock < FxHashMap < DepNodeIndex , DepNode > > , }
};
}
