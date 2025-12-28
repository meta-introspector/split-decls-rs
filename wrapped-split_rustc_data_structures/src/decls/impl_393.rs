macro_rules! deps {
    () => {
        ShardedHashMap!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < K : Eq , V > ShardedHashMap < K , V > { pub fn with_capacity (cap : usize) -> Self { Self :: new (| | HashTable :: with_capacity (cap)) } pub fn len (& self) -> usize { self . lock_shards () . map (| shard | shard . len ()) . sum () } }
    };
}

impl_393!()