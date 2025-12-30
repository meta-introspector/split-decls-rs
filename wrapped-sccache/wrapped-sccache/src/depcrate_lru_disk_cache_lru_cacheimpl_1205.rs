// Generated macro for impl_1205 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1205 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1205"}
// Dependencies: {}
impl < K , V > Meter < K , V > for Count { # [doc = " Don't store anything, the measurement can be derived from the map."] type Measure = () ; # [doc = " Don't actually count anything either."] fn measure < Q : ? Sized > (& self , _ : & Q , _ : & V) where K : Borrow < Q > , { } }
};
}
