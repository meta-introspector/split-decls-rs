// Generated macro for test_serde_set (function)
macro_rules! Depcratetest_serde_set {
() => {
// Module: crate
// Provides: {"test_serde_set"}
// Dependencies: {}
# [test] fn test_serde_set () { let set = indexset ! { 1 , 2 , 3 , 4 } ; assert_tokens (& set , & [Token :: Seq { len : Some (4) } , Token :: I32 (1) , Token :: I32 (2) , Token :: I32 (3) , Token :: I32 (4) , Token :: SeqEnd ,] ,) ; }
};
}
