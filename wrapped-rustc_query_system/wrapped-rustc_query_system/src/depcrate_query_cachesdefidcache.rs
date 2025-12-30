// Generated macro for DefIdCache (struct)
macro_rules! Depcrate_query_cachesDefIdCache {
() => {
// Module: crate::query::caches
// Provides: {"DefIdCache"}
// Dependencies: {}
# [doc = " In-memory cache for queries whose key is a [`DefId`]."] # [doc = ""] # [doc = " Selects between one of two internal caches, depending on whether the key"] # [doc = " is a local ID or foreign-crate ID."] pub struct DefIdCache < V > { # [doc = " Stores the local DefIds in a dense map. Local queries are much more often dense, so this is"] # [doc = " a win over hashing query keys at marginal memory cost (~5% at most) compared to FxHashMap."] local : VecCache < DefIndex , V , DepNodeIndex > , foreign : DefaultCache < DefId , V > , }
};
}
