macro_rules! deps {
    () => {
        RwLock!();
    };
}

macro_rules! SelfProfiler {
    () => {
        deps!();
        pub struct SelfProfiler { profiler : Profiler , event_filter_mask : EventFilter , string_cache : RwLock < FxHashMap < String , StringId > > , # [doc = " Recording individual query cache hits as \"instant\" measureme events"] # [doc = " is incredibly expensive. Instead of doing that, we simply aggregate"] # [doc = " cache hit *counts* per query invocation, and then store the final count"] # [doc = " of cache hits per invocation at the end of the compilation session."] # [doc = ""] # [doc = " With this approach, we don't know the individual thread IDs and timestamps"] # [doc = " of cache hits, but it has very little overhead on top of `-Zself-profile`."] # [doc = " Recording the cache hits as individual events made compilation 3-5x slower."] # [doc = ""] # [doc = " Query invocation IDs should be monotonic integers, so we can store them in a vec,"] # [doc = " rather than using a hashmap."] query_hits : RwLock < Vec < AtomicU64 > > , query_event_kind : StringId , generic_activity_event_kind : StringId , incremental_load_result_event_kind : StringId , incremental_result_hashing_event_kind : StringId , query_blocked_event_kind : StringId , query_cache_hit_event_kind : StringId , artifact_size_event_kind : StringId , # [doc = " Total cache hits per query invocation"] query_cache_hit_count_event_kind : StringId , }
    };
}

SelfProfiler!()