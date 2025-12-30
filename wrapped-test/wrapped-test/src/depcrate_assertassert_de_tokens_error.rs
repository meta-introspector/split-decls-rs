// Generated macro for assert_de_tokens_error (function)
macro_rules! Depcrate_assertassert_de_tokens_error {
() => {
// Module: crate::assert
// Provides: {"assert_de_tokens_error"}
// Dependencies: {}
# [doc = " Asserts that the given `tokens` yield `error` when deserializing."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_derive::{Deserialize, Serialize};"] # [doc = " # use serde_test::{assert_de_tokens_error, Token};"] # [doc = " #"] # [doc = " #[derive(Serialize, Deserialize, PartialEq, Debug)]"] # [doc = " #[serde(deny_unknown_fields)]"] # [doc = " struct S {"] # [doc = "     a: u8,"] # [doc = "     b: u8,"] # [doc = " }"] # [doc = ""] # [doc = " assert_de_tokens_error::<S>("] # [doc = "     &["] # [doc = "         Token::Struct { name: \"S\", len: 2 },"] # [doc = "         Token::Str(\"x\"),"] # [doc = "     ],"] # [doc = "     \"unknown field `x`, expected `a` or `b`\","] # [doc = " );"] # [doc = " ```"] # [track_caller] pub fn assert_de_tokens_error < 'de , T > (tokens : & 'de [Token] , error : & str) where T : Deserialize < 'de > , { let mut de = Deserializer :: new (tokens) ; match T :: deserialize (& mut de) { Ok (_) => panic ! ("tokens deserialized successfully") , Err (e) => assert_eq ! (e , * error) , } de . next_token_opt () ; if de . remaining () > 0 { panic ! ("{} remaining tokens" , de . remaining ()) ; } }
};
}
