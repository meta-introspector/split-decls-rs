macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
    };
}

macro_rules! BlockedOnInner {
    () => {
        deps!();
        struct BlockedOnInner < 'me > { dg : crate :: sync :: MutexGuard < 'me , DependencyGraph > , query_mutex_guard : SyncGuard < 'me > , database_key : DatabaseKeyIndex , other_id : ThreadId , thread_id : ThreadId , }
    };
}

BlockedOnInner!()