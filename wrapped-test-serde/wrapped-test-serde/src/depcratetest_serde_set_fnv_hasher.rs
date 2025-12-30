// Generated macro for test_serde_set_fnv_hasher (function)
macro_rules! Depcratetest_serde_set_fnv_hasher {
() => {
// Module: crate
// Provides: {"test_serde_set_fnv_hasher"}
// Dependencies: {}
# [test] fn test_serde_set_fnv_hasher () { let mut set : IndexSet < i32 , FnvBuildHasher > = Default :: default () ; set . extend (1 .. 5) ; assert_tokens (& set , & [Token :: Seq { len : Some (4) } , Token :: I32 (1) , Token :: I32 (2) , Token :: I32 (3) , Token :: I32 (4) , Token :: SeqEnd ,] ,) ; }
};
}
