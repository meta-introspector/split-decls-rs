// Generated macro for test_serde_seq_map (function)
macro_rules! Depcratetest_serde_seq_map {
() => {
// Module: crate
// Provides: {"test_serde_seq_map"}
// Dependencies: {}
# [test] fn test_serde_seq_map () { # [derive (Debug , Deserialize , Serialize)] # [serde (transparent)] struct SeqIndexMap { # [serde (with = "indexmap::map::serde_seq")] map : IndexMap < i32 , i32 > , } impl PartialEq for SeqIndexMap { fn eq (& self , other : & Self) -> bool { self . map . iter () . eq (& other . map) } } let map = indexmap ! { 1 => 2 , 3 => 4 , - 1 => - 2 , - 3 => - 4 } ; assert_tokens (& SeqIndexMap { map } , & [Token :: Seq { len : Some (4) } , Token :: Tuple { len : 2 } , Token :: I32 (1) , Token :: I32 (2) , Token :: TupleEnd , Token :: Tuple { len : 2 } , Token :: I32 (3) , Token :: I32 (4) , Token :: TupleEnd , Token :: Tuple { len : 2 } , Token :: I32 (- 1) , Token :: I32 (- 2) , Token :: TupleEnd , Token :: Tuple { len : 2 } , Token :: I32 (- 3) , Token :: I32 (- 4) , Token :: TupleEnd , Token :: SeqEnd ,] ,) ; }
};
}
