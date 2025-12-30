// Generated macro for ProvisionalCacheEntry (struct)
macro_rules! Depcrate_search_graphProvisionalCacheEntry {
() => {
// Module: crate::search_graph
// Provides: {"ProvisionalCacheEntry"}
// Dependencies: {}
# [doc = " A provisional result of an already computed goals which depends on other"] # [doc = " goals still on the stack."] # [derive_where (Debug ; X : Cx)] struct ProvisionalCacheEntry < X : Cx > { # [doc = " Whether evaluating the goal encountered overflow. This is used to"] # [doc = " disable the cache entry except if the last goal on the stack is"] # [doc = " already involved in this cycle."] encountered_overflow : bool , # [doc = " All cycle heads this cache entry depends on."] heads : CycleHeads , # [doc = " The path from the highest cycle head to this goal. This differs from"] # [doc = " `heads` which tracks the path to the cycle head *from* this goal."] path_from_head : PathKind , result : X :: Result , }
};
}
