// Generated macro for Inner (struct)
macro_rules! Depcrate_graph_dominatorsInner {
() => {
// Module: crate::graph::dominators
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Tracks the list of dominators for each node."] # [derive (Clone , Debug)] struct Inner < N : Idx > { immediate_dominators : IndexVec < N , Option < N > > , time : IndexVec < N , Time > , }
};
}
