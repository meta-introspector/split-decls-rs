// Generated macro for Predecessors (trait)
macro_rules! Depcrate_graphPredecessors {
() => {
// Module: crate::graph
// Provides: {"Predecessors"}
// Dependencies: {}
pub trait Predecessors : DirectedGraph { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }
};
}
