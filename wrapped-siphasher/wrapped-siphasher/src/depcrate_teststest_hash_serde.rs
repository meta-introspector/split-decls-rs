// Generated macro for test_hash_serde (function)
macro_rules! Depcrate_teststest_hash_serde {
() => {
// Module: crate::tests
// Provides: {"test_hash_serde"}
// Dependencies: {}
# [test] # [cfg (all (feature = "serde" , feature = "serde_json"))] fn test_hash_serde () { let val64 = 0xdead_beef_dead_beef_u64 ; let hash = hash (& val64) ; let serialized = serde_json :: to_string (& hash) . unwrap () ; let deserialized : u64 = serde_json :: from_str (& serialized) . unwrap () ; assert_eq ! (hash , deserialized) ; }
};
}
