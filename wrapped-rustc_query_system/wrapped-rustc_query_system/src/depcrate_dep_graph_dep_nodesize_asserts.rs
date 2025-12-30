// Generated macro for size_asserts (module)
macro_rules! Depcrate_dep_graph_dep_nodesize_asserts {
() => {
// Module: crate::dep_graph::dep_node
// Provides: {"size_asserts"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (DepKind , 2) ; # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] static_assert_size ! (DepNode , 18) ; # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] static_assert_size ! (DepNode , 24) ; }
};
}
