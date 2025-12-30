// Generated macro for assert_de_tokens (function)
macro_rules! Depcrate_assertassert_de_tokens {
() => {
// Module: crate::assert
// Provides: {"assert_de_tokens"}
// Dependencies: {}
# [doc = " Asserts that the given `tokens` deserialize into `value`."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_derive::{Deserialize, Serialize};"] # [doc = " # use serde_test::{assert_de_tokens, Token};"] # [doc = " #"] # [doc = " #[derive(Serialize, Deserialize, PartialEq, Debug)]"] # [doc = " struct S {"] # [doc = "     a: u8,"] # [doc = "     b: u8,"] # [doc = " }"] # [doc = ""] # [doc = " let s = S { a: 0, b: 0 };"] # [doc = " assert_de_tokens("] # [doc = "     &s,"] # [doc = "     &["] # [doc = "         Token::Struct { name: \"S\", len: 2 },"] # [doc = "         Token::Str(\"a\"),"] # [doc = "         Token::U8(0),"] # [doc = "         Token::Str(\"b\"),"] # [doc = "         Token::U8(0),"] # [doc = "         Token::StructEnd,"] # [doc = "     ],"] # [doc = " );"] # [doc = " ```"] # [track_caller] pub fn assert_de_tokens < 'de , T > (value : & T , tokens : & 'de [Token]) where T : Deserialize < 'de > + PartialEq + Debug , { let mut de = Deserializer :: new (tokens) ; let mut deserialized_val = match T :: deserialize (& mut de) { Ok (v) => { assert_eq ! (v , * value) ; v } Err (e) => panic ! ("tokens failed to deserialize: {}" , e) , } ; if de . remaining () > 0 { panic ! ("{} remaining tokens" , de . remaining ()) ; } let mut de = Deserializer :: new (tokens) ; match T :: deserialize_in_place (& mut de , & mut deserialized_val) { Ok (()) => { assert_eq ! (deserialized_val , * value) ; } Err (e) => panic ! ("tokens failed to deserialize_in_place: {}" , e) , } if de . remaining () > 0 { panic ! ("{} remaining tokens" , de . remaining ()) ; } }
};
}
