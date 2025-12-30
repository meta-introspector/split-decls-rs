// Generated macro for impl_215 (impl)
macro_rules! Depcrateimpl_215 {
() => {
// Module: crate
// Provides: {"impl_215"}
// Dependencies: {}
impl < T > Slab < T > { # [doc = " Returns a new slab with the default configuration parameters."] pub fn new () -> Self { Self :: new_with_config () } # [doc = " Returns a new slab with the provided configuration parameters."] pub fn new_with_config < C : cfg :: Config > () -> Slab < T , C > { C :: validate () ; Slab { shards : shard :: Array :: new () , _cfg : PhantomData , } } }
};
}
