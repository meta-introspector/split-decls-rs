// Generated macro for each_edge (function)
macro_rules! Depcrate_graph_linked_graph_testseach_edge {
() => {
// Module: crate::graph::linked_graph::tests
// Provides: {"each_edge"}
// Dependencies: {}
# [test] fn each_edge () { let graph = create_graph () ; let expected = ["AB" , "BC" , "BD" , "DE" , "EC" , "FB"] ; graph . each_edge (| idx , edge | { assert_eq ! (expected [idx . 0] , edge . data) ; true }) ; }
};
}
