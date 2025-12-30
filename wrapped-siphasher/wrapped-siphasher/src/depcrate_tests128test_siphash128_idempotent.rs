// Generated macro for test_siphash128_idempotent (function)
macro_rules! Depcrate_tests128test_siphash128_idempotent {
() => {
// Module: crate::tests128
// Provides: {"test_siphash128_idempotent"}
// Dependencies: {}
# [test] fn test_siphash128_idempotent () { let val64 = 0xdead_beef_dead_beef_u64 ; assert_eq ! (hash (& val64) , hash (& val64)) ; let val32 = 0xdeadbeef_u32 ; assert_eq ! (hash (& val32) , hash (& val32)) ; }
};
}
