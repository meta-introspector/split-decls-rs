macro_rules! deps {
    () => {
        InterestCacheConfig!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for InterestCacheConfig { fn default () -> Self { InterestCacheConfig { min_verbosity : Level :: Debug , lru_cache_size : 1024 , } } }
    };
}

impl_10!()