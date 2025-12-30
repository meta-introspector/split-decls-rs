// Generated macro for default_dep_node_debug (function)
macro_rules! Depcrate_dep_graph_dep_nodedefault_dep_node_debug {
() => {
// Module: crate::dep_graph::dep_node
// Provides: {"default_dep_node_debug"}
// Dependencies: {}
pub fn default_dep_node_debug (node : DepNode , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DepNode") . field ("kind" , & node . kind) . field ("hash" , & node . hash) . finish () }
};
}
