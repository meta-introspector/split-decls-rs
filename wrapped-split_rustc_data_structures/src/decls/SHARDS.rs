macro_rules! SHARDS {
    () => {
        const SHARDS : usize = 1 << SHARD_BITS ;
    };
}

SHARDS!()