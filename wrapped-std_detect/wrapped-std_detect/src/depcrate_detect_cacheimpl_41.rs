// Generated macro for impl_41 (impl)
macro_rules! Depcrate_detect_cacheimpl_41 {
() => {
// Module: crate::detect::cache
// Provides: {"impl_41"}
// Dependencies: {}
impl Initializer { # [doc = " Tests the `bit` of the cache."] # [inline] pub (crate) fn test (self , bit : u32) -> bool { debug_assert ! (bit < CACHE_CAPACITY , "too many features, time to increase the cache size!") ; test_bit (self . 0 , bit) } # [doc = " Sets the `bit` of the cache."] # [inline] pub (crate) fn set (& mut self , bit : u32) { debug_assert ! (bit < CACHE_CAPACITY , "too many features, time to increase the cache size!") ; let v = self . 0 ; self . 0 = set_bit (v , bit) ; } # [doc = " Unsets the `bit` of the cache."] # [inline] pub (crate) fn unset (& mut self , bit : u32) { debug_assert ! (bit < CACHE_CAPACITY , "too many features, time to increase the cache size!") ; let v = self . 0 ; self . 0 = unset_bit (v , bit) ; } }
};
}
