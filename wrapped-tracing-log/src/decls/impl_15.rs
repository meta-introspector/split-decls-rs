macro_rules! deps {
    () => {
        InterestCacheConfig!();
        State!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl State { fn new (epoch : usize , config : & InterestCacheConfig) -> Self { State { epoch , min_verbosity : config . min_verbosity , cache : LruCache :: new (config . lru_cache_size) , } } }
    };
}

impl_15!();