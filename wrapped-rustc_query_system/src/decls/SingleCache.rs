macro_rules! SingleCache {
    () => {
        # [doc = " In-memory cache for queries whose key type only has one value (e.g. `()`)."] # [doc = " The cache therefore only needs to store one query return value."] pub struct SingleCache < V > { cache : OnceLock < (V , DepNodeIndex) > , }
    };
}

SingleCache!()