// Generated macro for assert_next_token (function)
macro_rules! Depcrate_deassert_next_token {
() => {
// Module: crate::de
// Provides: {"assert_next_token"}
// Dependencies: {}
fn assert_next_token (de : & mut Deserializer , expected : Token) -> Result < () , Error > { match de . next_token_opt () { Some (token) if token == expected => Ok (()) , Some (other) => Err (de :: Error :: custom (format ! ("expected Token::{} but deserialization wants Token::{}" , other , expected ,))) , None => Err (de :: Error :: custom (format ! ("end of tokens but deserialization wants Token::{}" , expected ,))) , } }
};
}
