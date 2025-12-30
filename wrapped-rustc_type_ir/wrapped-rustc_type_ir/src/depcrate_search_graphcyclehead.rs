// Generated macro for CycleHead (struct)
macro_rules! Depcrate_search_graphCycleHead {
() => {
// Module: crate::search_graph
// Provides: {"CycleHead"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] struct CycleHead { paths_to_head : PathsToNested , # [doc = " If the `usages` are empty, the result of that head does not matter"] # [doc = " for the current goal. However, we still don't completely drop this"] # [doc = " cycle head as whether or not it exists impacts which queries we"] # [doc = " access, so ignoring it would cause incremental compilation verification"] # [doc = " failures or hide query cycles."] usages : HeadUsages , }
};
}
