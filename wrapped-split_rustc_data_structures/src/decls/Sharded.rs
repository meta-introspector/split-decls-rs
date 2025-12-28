macro_rules! deps {
    () => {
        CacheAligned!();
    };
}

macro_rules! Sharded {
    () => {
        deps!();
        # [doc = " An array of cache-line aligned inner locked structures with convenience methods."] # [doc = " A single field is used when the compiler uses only one thread."] pub enum Sharded < T > { Single (Lock < T >) , Shards (Box < [CacheAligned < Lock < T > > ; SHARDS] >) , }
    };
}

Sharded!()