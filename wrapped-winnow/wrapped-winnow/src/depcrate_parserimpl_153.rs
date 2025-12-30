// Generated macro for impl_153 (impl)
macro_rules! Depcrate_parserimpl_153 {
() => {
// Module: crate::parser
// Provides: {"impl_153"}
// Dependencies: {}
# [doc = " This is a shortcut for [`one_of`][crate::token::one_of]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::{error::ErrMode, error::ContextError};"] # [doc = " fn parser<'s>(i: &mut &'s [u8]) -> ModalResult<u8>  {"] # [doc = "     b'a'.parse_next(i)"] # [doc = " }"] # [doc = " assert_eq!(parser.parse_peek(&b\"abc\"[..]), Ok((&b\"bc\"[..], b'a')));"] # [doc = " assert!(parser.parse_peek(&b\" abc\"[..]).is_err());"] # [doc = " assert!(parser.parse_peek(&b\"bc\"[..]).is_err());"] # [doc = " assert!(parser.parse_peek(&b\"\"[..]).is_err());"] # [doc = " ```"] impl < I , E > Parser < I , u8 , E > for u8 where I : StreamIsPartial , I : Stream , I : Compare < u8 > , E : ParserError < I > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < u8 , E > { crate :: token :: literal (* self) . value (* self) . parse_next (i) } }
};
}
