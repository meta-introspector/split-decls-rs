// Generated macro for DepthFirstTraversal (struct)
macro_rules! Depcrate_graph_linked_graphDepthFirstTraversal {
() => {
// Module: crate::graph::linked_graph
// Provides: {"DepthFirstTraversal"}
// Dependencies: {}
pub struct DepthFirstTraversal < 'g , N , E > { graph : & 'g LinkedGraph < N , E > , stack : Vec < NodeIndex > , visited : DenseBitSet < usize > , direction : Direction , }
};
}
