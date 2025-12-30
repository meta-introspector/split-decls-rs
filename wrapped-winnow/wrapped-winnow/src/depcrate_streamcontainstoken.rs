// Generated macro for ContainsToken (trait)
macro_rules! Depcrate_streamContainsToken {
() => {
// Module: crate::stream
// Provides: {"ContainsToken"}
// Dependencies: {}
# [doc = " Check if a token is in a set of possible tokens"] # [doc = ""] # [doc = " While this can be implemented manually, you can also build up sets using:"] # [doc = " - `b'c'` and `'c'`"] # [doc = " - `b\"\"`"] # [doc = " - `|c| true`"] # [doc = " - `b'a'..=b'z'`, `'a'..='z'` (etc for each [range type][std::ops])"] # [doc = " - `(set1, set2, ...)`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " For example, you could implement `hex_digit0` as:"] # [doc = " ```"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::{error::ErrMode, error::ContextError};"] # [doc = " # use winnow::token::take_while;"] # [doc = " fn hex_digit1<'s>(input: &mut &'s str) -> ModalResult<&'s str, ContextError> {"] # [doc = "     take_while(1.., ('a'..='f', 'A'..='F', '0'..='9')).parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(hex_digit1.parse_peek(\"21cZ\"), Ok((\"Z\", \"21c\")));"] # [doc = " assert!(hex_digit1.parse_peek(\"H2\").is_err());"] # [doc = " assert!(hex_digit1.parse_peek(\"\").is_err());"] # [doc = " ```"] pub trait ContainsToken < T > { # [doc = " Returns true if self contains the token"] fn contains_token (& self , token : T) -> bool ; }
};
}
