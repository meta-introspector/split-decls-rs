// Generated macro for TestToken (enum)
macro_rules! Depcrate_testTestToken {
() => {
// Module: crate::test
// Provides: {"TestToken"}
// Dependencies: {}
# [doc = "\nA tokenized representation of the captured value for testing.\n"] # [derive (Debug , PartialEq)] # [non_exhaustive] pub enum TestToken { U64 (u64) , I64 (i64) , F64 (f64) , U128 (u128) , I128 (i128) , Char (char) , Bool (bool) , Str (String) , None , # [cfg (feature = "error")] Error , # [cfg (feature = "sval2")] Sval { version : u32 , } , # [cfg (feature = "serde1")] Serde { version : u32 , } , # [cfg (feature = "seq")] Seq , Poisoned (String) , }
};
}
