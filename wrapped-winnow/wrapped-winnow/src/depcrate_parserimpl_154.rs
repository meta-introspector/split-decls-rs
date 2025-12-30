// Generated macro for impl_154 (impl)
macro_rules! Depcrate_parserimpl_154 {
() => {
// Module: crate::parser
// Provides: {"impl_154"}
// Dependencies: {}
# [doc = " This is a shortcut for [`one_of`][crate::token::one_of]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::{error::ErrMode, error::ContextError};"] # [doc = " fn parser<'s>(i: &mut &'s str) -> ModalResult<char> {"] # [doc = "     'a'.parse_next(i)"] # [doc = " }"] # [doc = " assert_eq!(parser.parse_peek(\"abc\"), Ok((\"bc\", 'a')));"] # [doc = " assert!(parser.parse_peek(\" abc\").is_err());"] # [doc = " assert!(parser.parse_peek(\"bc\").is_err());"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " ```"] impl < I , E > Parser < I , char , E > for char where I : StreamIsPartial , I : Stream , I : Compare < char > , E : ParserError < I > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < char , E > { crate :: token :: literal (* self) . value (* self) . parse_next (i) } }
};
}
