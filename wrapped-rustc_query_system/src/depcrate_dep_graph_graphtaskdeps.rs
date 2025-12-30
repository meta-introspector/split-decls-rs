// Generated macro for TaskDeps (struct)
macro_rules! Depcrate_dep_graph_graphTaskDeps {
() => {
// Module: crate::dep_graph::graph
// Provides: {"TaskDeps"}
// Dependencies: {}
# [derive (Debug)] pub struct TaskDeps { # [cfg (debug_assertions)] node : Option < DepNode > , reads : EdgesVec , read_set : FxHashSet < DepNodeIndex > , phantom_data : PhantomData < DepNode > , }
};
}
