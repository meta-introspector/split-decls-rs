// Generated macro for paper (function)
macro_rules! Depcrate_graph_dominators_testspaper {
() => {
// Module: crate::graph::dominators::tests
// Provides: {"paper"}
// Dependencies: {}
# [test] fn paper () { let graph = TestGraph :: new (6 , & [(6 , 5) , (6 , 4) , (5 , 1) , (4 , 2) , (4 , 3) , (1 , 2) , (2 , 3) , (3 , 2) , (2 , 1)] ,) ; let d = dominators (& graph) ; assert_eq ! (d . immediate_dominator (0) , None) ; assert_eq ! (d . immediate_dominator (1) , Some (6)) ; assert_eq ! (d . immediate_dominator (2) , Some (6)) ; assert_eq ! (d . immediate_dominator (3) , Some (6)) ; assert_eq ! (d . immediate_dominator (4) , Some (6)) ; assert_eq ! (d . immediate_dominator (5) , Some (6)) ; assert_eq ! (d . immediate_dominator (6) , None) ; }
};
}
