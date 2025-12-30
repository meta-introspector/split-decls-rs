// Generated macro for test_sval_set_fnv_hasher (function)
macro_rules! Depcratetest_sval_set_fnv_hasher {
() => {
// Module: crate
// Provides: {"test_sval_set_fnv_hasher"}
// Dependencies: {}
# [test] fn test_sval_set_fnv_hasher () { let mut set : IndexSet < i32 , FnvBuildHasher > = Default :: default () ; set . extend (1 .. 5) ; assert_tokens (& set , & [Token :: SeqBegin (Some (4)) , Token :: SeqValueBegin , Token :: I32 (1) , Token :: SeqValueEnd , Token :: SeqValueBegin , Token :: I32 (2) , Token :: SeqValueEnd , Token :: SeqValueBegin , Token :: I32 (3) , Token :: SeqValueEnd , Token :: SeqValueBegin , Token :: I32 (4) , Token :: SeqValueEnd , Token :: SeqEnd ,] ,) ; }
};
}
