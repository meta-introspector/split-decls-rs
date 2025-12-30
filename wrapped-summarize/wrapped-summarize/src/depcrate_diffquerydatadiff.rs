// Generated macro for QueryDataDiff (struct)
macro_rules! Depcrate_diffQueryDataDiff {
() => {
// Module: crate::diff
// Provides: {"QueryDataDiff"}
// Dependencies: {}
# [doc = " The diff between two `QueryData`"] # [derive (Serialize , Deserialize)] pub struct QueryDataDiff { pub label : String , pub time : SignedDuration , pub time_change : f64 , pub self_time : SignedDuration , pub self_time_change : f64 , pub number_of_cache_misses : i64 , pub number_of_cache_hits : i64 , pub invocation_count : i64 , pub blocked_time : SignedDuration , pub incremental_load_time : SignedDuration , pub incremental_hashing_time : SignedDuration , }
};
}
