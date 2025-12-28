macro_rules! deps {
    () => {
        InterestCacheConfig!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl InterestCacheConfig { fn disabled () -> Self { Self { lru_cache_size : 0 , .. Self :: default () } } }
    };
}

impl_11!()