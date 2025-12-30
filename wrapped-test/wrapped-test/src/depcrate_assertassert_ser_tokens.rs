// Generated macro for assert_ser_tokens (function)
macro_rules! Depcrate_assertassert_ser_tokens {
() => {
// Module: crate::assert
// Provides: {"assert_ser_tokens"}
// Dependencies: {}
# [doc = " Asserts that `value` serializes to the given `tokens`."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_derive::{Deserialize, Serialize};"] # [doc = " # use serde_test::{assert_ser_tokens, Token};"] # [doc = " #"] # [doc = " #[derive(Serialize, Deserialize, PartialEq, Debug)]"] # [doc = " struct S {"] # [doc = "     a: u8,"] # [doc = "     b: u8,"] # [doc = " }"] # [doc = ""] # [doc = " let s = S { a: 0, b: 0 };"] # [doc = " assert_ser_tokens("] # [doc = "     &s,"] # [doc = "     &["] # [doc = "         Token::Struct { name: \"S\", len: 2 },"] # [doc = "         Token::Str(\"a\"),"] # [doc = "         Token::U8(0),"] # [doc = "         Token::Str(\"b\"),"] # [doc = "         Token::U8(0),"] # [doc = "         Token::StructEnd,"] # [doc = "     ],"] # [doc = " );"] # [doc = " ```"] # [track_caller] pub fn assert_ser_tokens < T > (value : & T , tokens : & [Token]) where T : ? Sized + Serialize , { let mut ser = Serializer :: new (tokens) ; match value . serialize (& mut ser) { Ok (()) => { } Err (err) => panic ! ("value failed to serialize: {}" , err) , } if ser . remaining () > 0 { panic ! ("{} remaining tokens" , ser . remaining ()) ; } }
};
}
