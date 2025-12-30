// Generated macro for DepGraphQuery (struct)
macro_rules! Depcrate_dep_graph_queryDepGraphQuery {
() => {
// Module: crate::dep_graph::query
// Provides: {"DepGraphQuery"}
// Dependencies: {}
pub struct DepGraphQuery { pub graph : LinkedGraph < DepNode , () > , pub indices : FxHashMap < DepNode , NodeIndex > , pub dep_index_to_index : IndexVec < DepNodeIndex , Option < NodeIndex > > , }
};
}
