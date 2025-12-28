macro_rules! deps {
    () => {
        InterestCacheConfig!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl InterestCacheConfig { # [doc = " Sets the minimum logging verbosity for which the cache will apply."] # [doc = ""] # [doc = " The interest for logs with a lower verbosity than specified here"] # [doc = " will not be cached."] # [doc = ""] # [doc = " It should be set to the lowest verbosity level for which the majority"] # [doc = " of the logs in your application are usually *disabled*."] # [doc = ""] # [doc = " In normal circumstances with typical logger usage patterns"] # [doc = " you shouldn't ever have to change this."] # [doc = ""] # [doc = " By default this is set to `Debug`."] pub fn with_min_verbosity (mut self , level : Level) -> Self { self . min_verbosity = level ; self } # [doc = " Sets the number of entries in the LRU cache used to cache interests"] # [doc = " for `log` records."] # [doc = ""] # [doc = " The bigger the cache, the more unlikely it will be for the interest"] # [doc = " in a given callsite to be recalculated, at the expense of extra"] # [doc = " memory usage per every thread which tries to log events."] # [doc = ""] # [doc = " Every unique [level] + [target] pair consumes a single slot"] # [doc = " in the cache. Entries will be added to the cache until its size"] # [doc = " reaches the value configured here, and from then on it will evict"] # [doc = " the least recently seen level + target pair when adding a new entry."] # [doc = ""] # [doc = " The ideal value to set here widely depends on how much exactly"] # [doc = " you're logging, and how diverse the targets are to which you are logging."] # [doc = ""] # [doc = " If your application spends a significant amount of time filtering logs"] # [doc = " which are *not* getting printed out then increasing this value will most"] # [doc = " likely help."] # [doc = ""] # [doc = " Setting this to zero will disable the cache."] # [doc = ""] # [doc = " By default this is set to 1024."] # [doc = ""] # [doc = " [level]: log::Metadata::level"] # [doc = " [target]: log::Metadata::target"] pub fn with_lru_cache_size (mut self , size : usize) -> Self { self . lru_cache_size = size ; self } }
    };
}

impl_12!()