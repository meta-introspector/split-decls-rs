// Generated macro for BlockedOnInner (struct)
macro_rules! Depcrate_runtimeBlockedOnInner {
() => {
// Module: crate::runtime
// Provides: {"BlockedOnInner"}
// Dependencies: {}
struct BlockedOnInner < 'me > { dg : crate :: sync :: MutexGuard < 'me , DependencyGraph > , query_mutex_guard : SyncGuard < 'me > , database_key : DatabaseKeyIndex , other_id : ThreadId , thread_id : ThreadId , }
};
}
