// Generated macro for test_siphash128_serde (function)
macro_rules! Depcrate_tests128test_siphash128_serde {
() => {
// Module: crate::tests128
// Provides: {"test_siphash128_serde"}
// Dependencies: {}
# [test] # [cfg (all (feature = "serde" , feature = "serde_json"))] fn test_siphash128_serde () { let val64 = 0xdead_beef_dead_beef_u64 ; let hash = hash (& val64) ; let serialized = serde_json :: to_string (& hash) . unwrap () ; let deserialized : [u8 ; 16] = serde_json :: from_str (& serialized) . unwrap () ; assert_eq ! (hash , deserialized) ; }
};
}
