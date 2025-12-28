macro_rules! DefaultCache {
    () => {
        # [doc = " In-memory cache for queries whose keys aren't suitable for any of the"] # [doc = " more specialized kinds of cache. Backed by a sharded hashmap."] pub struct DefaultCache < K , V > { cache : ShardedHashMap < K , (V , DepNodeIndex) > , }
    };
}

DefaultCache!();