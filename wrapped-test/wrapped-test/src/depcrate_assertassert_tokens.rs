// Generated macro for assert_tokens (function)
macro_rules! Depcrate_assertassert_tokens {
() => {
// Module: crate::assert
// Provides: {"assert_tokens"}
// Dependencies: {}
# [doc = " Runs both `assert_ser_tokens` and `assert_de_tokens`."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_derive::{Deserialize, Serialize};"] # [doc = " # use serde_test::{assert_tokens, Token};"] # [doc = " #"] # [doc = " #[derive(Serialize, Deserialize, PartialEq, Debug)]"] # [doc = " struct S {"] # [doc = "     a: u8,"] # [doc = "     b: u8,"] # [doc = " }"] # [doc = ""] # [doc = " let s = S { a: 0, b: 0 };"] # [doc = " assert_tokens("] # [doc = "     &s,"] # [doc = "     &["] # [doc = "         Token::Struct { name: \"S\", len: 2 },"] # [doc = "         Token::Str(\"a\"),"] # [doc = "         Token::U8(0),"] # [doc = "         Token::Str(\"b\"),"] # [doc = "         Token::U8(0),"] # [doc = "         Token::StructEnd,"] # [doc = "     ],"] # [doc = " );"] # [doc = " ```"] # [track_caller] pub fn assert_tokens < 'de , T > (value : & T , tokens : & 'de [Token]) where T : Serialize + Deserialize < 'de > + PartialEq + Debug , { assert_ser_tokens (value , tokens) ; assert_de_tokens (value , tokens) ; }
};
}
