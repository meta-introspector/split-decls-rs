// Generated macro for impl_245 (impl)
macro_rules! Depcrate_tests_custom_configimpl_245 {
() => {
// Module: crate::tests::custom_config
// Provides: {"impl_245"}
// Dependencies: {}
# [cfg (not (target_pointer_width = "64"))] impl Config for CustomConfig { const INITIAL_PAGE_SIZE : usize = 16 ; const MAX_PAGES : usize = 6 ; const MAX_THREADS : usize = 128 ; const RESERVED_BITS : usize = 12 ; }
};
}
