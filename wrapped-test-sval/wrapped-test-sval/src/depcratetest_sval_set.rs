// Generated macro for test_sval_set (function)
macro_rules! Depcratetest_sval_set {
() => {
// Module: crate
// Provides: {"test_sval_set"}
// Dependencies: {}
# [test] fn test_sval_set () { let set = indexset ! { 1 , 2 , 3 , 4 } ; assert_tokens (& set , & [Token :: SeqBegin (Some (4)) , Token :: SeqValueBegin , Token :: I32 (1) , Token :: SeqValueEnd , Token :: SeqValueBegin , Token :: I32 (2) , Token :: SeqValueEnd , Token :: SeqValueBegin , Token :: I32 (3) , Token :: SeqValueEnd , Token :: SeqValueBegin , Token :: I32 (4) , Token :: SeqValueEnd , Token :: SeqEnd ,] ,) ; }
};
}
