// Generated macro for DEP_KIND_DEBUG (static)
macro_rules! Depcrate_dep_graph_dep_nodeDEP_KIND_DEBUG {
() => {
// Module: crate::dep_graph::dep_node
// Provides: {"DEP_KIND_DEBUG"}
// Dependencies: {}
pub static DEP_KIND_DEBUG : AtomicRef < fn (DepKind , & mut fmt :: Formatter < '_ >) -> fmt :: Result > = AtomicRef :: new (& (default_dep_kind_debug as fn (_ , & mut fmt :: Formatter < '_ >) -> _)) ;
};
}
