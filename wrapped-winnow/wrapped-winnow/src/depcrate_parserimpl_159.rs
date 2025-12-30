// Generated macro for impl_159 (impl)
macro_rules! Depcrate_parserimpl_159 {
() => {
// Module: crate::parser
// Provides: {"impl_159"}
// Dependencies: {}
# [doc = " This is a shortcut for [`literal`][crate::token::literal]."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::{error::ErrMode, error::ContextError};"] # [doc = " # use winnow::combinator::alt;"] # [doc = " # use winnow::token::take;"] # [doc = ""] # [doc = " fn parser<'s>(s: &mut &'s str) -> ModalResult<&'s str> {"] # [doc = "   alt((\"Hello\", take(5usize))).parse_next(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"Hello, World!\"), Ok((\", World!\", \"Hello\")));"] # [doc = " assert_eq!(parser.parse_peek(\"Something\"), Ok((\"hing\", \"Somet\")));"] # [doc = " assert!(parser.parse_peek(\"Some\").is_err());"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " ```"] impl < 's , I , E : ParserError < I > > Parser < I , < I as Stream > :: Slice , E > for & 's str where I : Compare < & 's str > + StreamIsPartial , I : Stream , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < < I as Stream > :: Slice , E > { crate :: token :: literal (* self) . parse_next (i) } }
};
}
