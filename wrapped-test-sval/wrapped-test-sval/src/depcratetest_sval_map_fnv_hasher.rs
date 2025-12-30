// Generated macro for test_sval_map_fnv_hasher (function)
macro_rules! Depcratetest_sval_map_fnv_hasher {
() => {
// Module: crate
// Provides: {"test_sval_map_fnv_hasher"}
// Dependencies: {}
# [test] fn test_sval_map_fnv_hasher () { let mut map : IndexMap < i32 , i32 , FnvBuildHasher > = Default :: default () ; map . insert (1 , 2) ; map . insert (3 , 4) ; assert_tokens (& map , & [Token :: MapBegin (Some (2)) , Token :: MapKeyBegin , Token :: I32 (1) , Token :: MapKeyEnd , Token :: MapValueBegin , Token :: I32 (2) , Token :: MapValueEnd , Token :: MapKeyBegin , Token :: I32 (3) , Token :: MapKeyEnd , Token :: MapValueBegin , Token :: I32 (4) , Token :: MapValueEnd , Token :: MapEnd ,] ,) ; }
};
}
