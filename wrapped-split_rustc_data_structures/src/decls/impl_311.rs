macro_rules! deps {
    () => {
        ForestObligation!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < 'a > super :: ForestObligation for & 'a str { type CacheKey = & 'a str ; fn as_cache_key (& self) -> Self :: CacheKey { self } }
    };
}

impl_311!();