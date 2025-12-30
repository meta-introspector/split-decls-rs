// Generated macro for UpdateParentGoalCtxt (enum)
macro_rules! Depcrate_search_graphUpdateParentGoalCtxt {
() => {
// Module: crate::search_graph
// Provides: {"UpdateParentGoalCtxt"}
// Dependencies: {}
# [doc = " While [`SearchGraph::update_parent_goal`] can be mostly shared between"] # [doc = " ordinary nested goals/global cache hits and provisional cache hits,"] # [doc = " using the provisional cache should not add any nested goals."] # [doc = ""] # [doc = " `nested_goals` are only used when checking whether global cache entries"] # [doc = " are applicable. This only cares about whether a goal is actually accessed."] # [doc = " Given that the usage of the provisional cache is fully deterministic, we"] # [doc = " don't need to track the nested goals used while computing a provisional"] # [doc = " cache entry."] enum UpdateParentGoalCtxt < 'a , X : Cx > { Ordinary (& 'a NestedGoals < X >) , CycleOnStack (X :: Input) , ProvisionalCacheHit , }
};
}
