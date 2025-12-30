// Generated macro for is_cyclic (function)
macro_rules! Depcrate_graph_iterate_testsis_cyclic {
() => {
// Module: crate::graph::iterate::tests
// Provides: {"is_cyclic"}
// Dependencies: {}
# [test] fn is_cyclic () { use super :: super :: is_cyclic ; let diamond_acyclic = TestGraph :: new (0 , & [(0 , 1) , (0 , 2) , (1 , 3) , (2 , 3)]) ; let diamond_cyclic = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (2 , 3) , (3 , 0)]) ; assert ! (! is_cyclic (& diamond_acyclic)) ; assert ! (is_cyclic (& diamond_cyclic)) ; }
};
}
