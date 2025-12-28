macro_rules! Slab {
    () => {
        # [doc = " A sharded slab."] # [doc = ""] # [doc = " See the [crate-level documentation](crate) for details on using this type."] pub struct Slab < T , C : cfg :: Config = DefaultConfig > { shards : shard :: Array < Option < T > , C > , _cfg : PhantomData < C > , }
    };
}

Slab!()