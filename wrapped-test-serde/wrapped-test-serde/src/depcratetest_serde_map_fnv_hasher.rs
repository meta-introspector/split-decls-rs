// Generated macro for test_serde_map_fnv_hasher (function)
macro_rules! Depcratetest_serde_map_fnv_hasher {
() => {
// Module: crate
// Provides: {"test_serde_map_fnv_hasher"}
// Dependencies: {}
# [test] fn test_serde_map_fnv_hasher () { let mut map : IndexMap < i32 , i32 , FnvBuildHasher > = Default :: default () ; map . insert (1 , 2) ; map . insert (3 , 4) ; assert_tokens (& map , & [Token :: Map { len : Some (2) } , Token :: I32 (1) , Token :: I32 (2) , Token :: I32 (3) , Token :: I32 (4) , Token :: MapEnd ,] ,) ; }
};
}
