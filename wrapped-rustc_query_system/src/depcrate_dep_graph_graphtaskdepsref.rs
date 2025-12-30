// Generated macro for TaskDepsRef (enum)
macro_rules! Depcrate_dep_graph_graphTaskDepsRef {
() => {
// Module: crate::dep_graph::graph
// Provides: {"TaskDepsRef"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub enum TaskDepsRef < 'a > { # [doc = " New dependencies can be added to the"] # [doc = " `TaskDeps`. This is used when executing a 'normal' query"] # [doc = " (no `eval_always` modifier)"] Allow (& 'a Lock < TaskDeps >) , # [doc = " This is used when executing an `eval_always` query. We don't"] # [doc = " need to track dependencies for a query that's always"] # [doc = " re-executed -- but we need to know that this is an `eval_always`"] # [doc = " query in order to emit dependencies to `DepNodeIndex::FOREVER_RED_NODE`"] # [doc = " when directly feeding other queries."] EvalAlways , # [doc = " New dependencies are ignored. This is also used for `dep_graph.with_ignore`."] Ignore , # [doc = " Any attempt to add new dependencies will cause a panic."] # [doc = " This is used when decoding a query result from disk,"] # [doc = " to ensure that the decoding process doesn't itself"] # [doc = " require the execution of any queries."] Forbid , }
};
}
