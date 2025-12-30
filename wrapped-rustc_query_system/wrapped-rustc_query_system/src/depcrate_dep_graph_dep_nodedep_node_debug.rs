// Generated macro for DEP_NODE_DEBUG (static)
macro_rules! Depcrate_dep_graph_dep_nodeDEP_NODE_DEBUG {
() => {
// Module: crate::dep_graph::dep_node
// Provides: {"DEP_NODE_DEBUG"}
// Dependencies: {}
pub static DEP_NODE_DEBUG : AtomicRef < fn (DepNode , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_dep_node_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
};
}
