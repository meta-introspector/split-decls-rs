// Generated macro for DepGraph (struct)
macro_rules! Depcrate_dep_graph_graphDepGraph {
() => {
// Module: crate::dep_graph::graph
// Provides: {"DepGraph"}
// Dependencies: {}
# [derive (Clone)] pub struct DepGraph < D : Deps > { data : Option < Arc < DepGraphData < D > > > , # [doc = " This field is used for assigning DepNodeIndices when running in"] # [doc = " non-incremental mode. Even in non-incremental mode we make sure that"] # [doc = " each task has a `DepNodeIndex` that uniquely identifies it. This unique"] # [doc = " ID is used for self-profiling."] virtual_dep_node_index : Arc < AtomicU32 > , }
};
}
