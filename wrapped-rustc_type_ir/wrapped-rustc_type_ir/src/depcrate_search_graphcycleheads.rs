// Generated macro for CycleHeads (struct)
macro_rules! Depcrate_search_graphCycleHeads {
() => {
// Module: crate::search_graph
// Provides: {"CycleHeads"}
// Dependencies: {}
# [doc = " All cycle heads a given goal depends on, ordered by their stack depth."] # [doc = ""] # [doc = " We also track all paths from this goal to that head. This is necessary"] # [doc = " when rebasing provisional cache results."] # [derive (Clone , Debug , Default)] struct CycleHeads { heads : BTreeMap < StackDepth , CycleHead > , }
};
}
