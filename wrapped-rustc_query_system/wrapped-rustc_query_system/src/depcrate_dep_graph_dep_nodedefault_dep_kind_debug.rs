// Generated macro for default_dep_kind_debug (function)
macro_rules! Depcrate_dep_graph_dep_nodedefault_dep_kind_debug {
() => {
// Module: crate::dep_graph::dep_node
// Provides: {"default_dep_kind_debug"}
// Dependencies: {}
pub fn default_dep_kind_debug (kind : DepKind , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DepKind") . field ("variant" , & kind . variant) . finish () }
};
}
