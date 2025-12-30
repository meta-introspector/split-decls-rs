// Generated macro for impl_158 (impl)
macro_rules! Depcrate_parserimpl_158 {
() => {
// Module: crate::parser
// Provides: {"impl_158"}
// Dependencies: {}
# [doc = " This is a shortcut for [`literal`][crate::token::literal]."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::{error::ErrMode, error::ContextError, error::Needed};"] # [doc = " # use winnow::combinator::alt;"] # [doc = " # use winnow::token::take;"] # [doc = " use winnow::ascii::Caseless;"] # [doc = ""] # [doc = " fn parser<'s>(s: &mut &'s [u8]) -> ModalResult<&'s [u8]> {"] # [doc = "   alt((Caseless(b\"hello\"), take(5usize))).parse_next(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(&b\"Hello, World!\"[..]), Ok((&b\", World!\"[..], &b\"Hello\"[..])));"] # [doc = " assert_eq!(parser.parse_peek(&b\"hello, World!\"[..]), Ok((&b\", World!\"[..], &b\"hello\"[..])));"] # [doc = " assert_eq!(parser.parse_peek(&b\"HeLlo, World!\"[..]), Ok((&b\", World!\"[..], &b\"HeLlo\"[..])));"] # [doc = " assert_eq!(parser.parse_peek(&b\"Something\"[..]), Ok((&b\"hing\"[..], &b\"Somet\"[..])));"] # [doc = " assert!(parser.parse_peek(&b\"Some\"[..]).is_err());"] # [doc = " assert!(parser.parse_peek(&b\"\"[..]).is_err());"] # [doc = " ```"] impl < 's , I , E : ParserError < I > , const N : usize > Parser < I , < I as Stream > :: Slice , E > for AsciiCaseless < & 's [u8 ; N] > where I : Compare < AsciiCaseless < & 's [u8 ; N] > > + StreamIsPartial , I : Stream , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < < I as Stream > :: Slice , E > { crate :: token :: literal (* self) . parse_next (i) } }
};
}
