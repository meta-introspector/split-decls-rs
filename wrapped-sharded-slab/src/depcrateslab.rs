// Generated macro for Slab (struct)
macro_rules! DepcrateSlab {
() => {
// Module: crate
// Provides: {"Slab"}
// Dependencies: {}
# [doc = " A sharded slab."] # [doc = ""] # [doc = " See the [crate-level documentation](crate) for details on using this type."] pub struct Slab < T , C : cfg :: Config = DefaultConfig > { shards : shard :: Array < Option < T > , C > , _cfg : PhantomData < C > , }
};
}
