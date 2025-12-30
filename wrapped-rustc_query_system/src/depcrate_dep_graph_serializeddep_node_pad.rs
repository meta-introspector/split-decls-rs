// Generated macro for DEP_NODE_PAD (const)
macro_rules! Depcrate_dep_graph_serializedDEP_NODE_PAD {
() => {
// Module: crate::dep_graph::serialized
// Provides: {"DEP_NODE_PAD"}
// Dependencies: {}
# [doc = " Amount of padding we need to add to the edge list data so that we can retrieve every"] # [doc = " SerializedDepNodeIndex with a fixed-size read then mask."] const DEP_NODE_PAD : usize = DEP_NODE_SIZE - 1 ;
};
}
