macro_rules! deps {
    () => {
        Sharded!();
    };
}

macro_rules! ShardedHashMap {
    () => {
        deps!();
        pub type ShardedHashMap < K , V > = Sharded < HashTable < (K , V) > > ;
    };
}

ShardedHashMap!();