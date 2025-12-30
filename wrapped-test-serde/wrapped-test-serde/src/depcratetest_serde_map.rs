// Generated macro for test_serde_map (function)
macro_rules! Depcratetest_serde_map {
() => {
// Module: crate
// Provides: {"test_serde_map"}
// Dependencies: {}
# [test] fn test_serde_map () { let map = indexmap ! { 1 => 2 , 3 => 4 } ; assert_tokens (& map , & [Token :: Map { len : Some (2) } , Token :: I32 (1) , Token :: I32 (2) , Token :: I32 (3) , Token :: I32 (4) , Token :: MapEnd ,] ,) ; }
};
}
