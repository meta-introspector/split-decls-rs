macro_rules! InterestCacheConfig {
    () => {
        # [doc = " The interest cache configuration."] # [derive (Debug)] pub struct InterestCacheConfig { min_verbosity : Level , lru_cache_size : usize , }
    };
}

InterestCacheConfig!();