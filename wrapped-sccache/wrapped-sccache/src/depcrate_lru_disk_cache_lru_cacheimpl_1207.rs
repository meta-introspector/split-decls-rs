// Generated macro for impl_1207 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1207 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1207"}
// Dependencies: {}
# [doc = " `Count` is all no-ops, the number of entries in the map is the size."] impl < K , V , T : Meter < K , V > > CountableMeter < K , V > for T where T : CountableMeterWithMeasure < K , V , < T as Meter < K , V > > :: Measure > , { fn add (& self , current : Self :: Measure , amount : Self :: Measure) -> Self :: Measure { CountableMeterWithMeasure :: meter_add (self , current , amount) } fn sub (& self , current : Self :: Measure , amount : Self :: Measure) -> Self :: Measure { CountableMeterWithMeasure :: meter_sub (self , current , amount) } fn size (& self , current : Self :: Measure) -> Option < u64 > { CountableMeterWithMeasure :: meter_size (self , current) } }
};
}
