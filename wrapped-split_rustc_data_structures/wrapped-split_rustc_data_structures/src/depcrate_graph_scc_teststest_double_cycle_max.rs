// Generated macro for test_double_cycle_max (function)
macro_rules! Depcrate_graph_scc_teststest_double_cycle_max {
() => {
// Module: crate::graph::scc::tests
// Provides: {"test_double_cycle_max"}
// Dependencies: {}
# [test] fn test_double_cycle_max () { let graph = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (1 , 4) , (2 , 3) , (2 , 4) , (3 , 5) , (4 , 1) , (5 , 4)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | if n == 5 { 2 } else { 1 }) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . 0 [sccs . scc (0)] . 0 , 2) ; }
};
}
