// Generated macro for test_sval_map (function)
macro_rules! Depcratetest_sval_map {
() => {
// Module: crate
// Provides: {"test_sval_map"}
// Dependencies: {}
# [test] fn test_sval_map () { let map = indexmap ! { 1 => 2 , 3 => 4 } ; assert_tokens (& map , & [Token :: MapBegin (Some (2)) , Token :: MapKeyBegin , Token :: I32 (1) , Token :: MapKeyEnd , Token :: MapValueBegin , Token :: I32 (2) , Token :: MapValueEnd , Token :: MapKeyBegin , Token :: I32 (3) , Token :: MapKeyEnd , Token :: MapValueBegin , Token :: I32 (4) , Token :: MapValueEnd , Token :: MapEnd ,] ,) ; }
};
}
