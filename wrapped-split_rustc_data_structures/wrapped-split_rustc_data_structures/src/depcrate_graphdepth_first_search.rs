// Generated macro for depth_first_search (function)
macro_rules! Depcrate_graphdepth_first_search {
() => {
// Module: crate::graph
// Provides: {"depth_first_search"}
// Dependencies: {}
pub fn depth_first_search < G > (graph : G , from : G :: Node) -> iterate :: DepthFirstSearch < G > where G : Successors , { iterate :: DepthFirstSearch :: new (graph) . with_start_node (from) }
};
}
