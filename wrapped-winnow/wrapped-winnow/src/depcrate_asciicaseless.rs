// Generated macro for Caseless (struct)
macro_rules! Depcrate_asciiCaseless {
() => {
// Module: crate::ascii
// Provides: {"Caseless"}
// Dependencies: {}
# [doc = " Mark a value as case-insensitive for ASCII characters"] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::ascii::Caseless;"] # [doc = ""] # [doc = " fn parser<'s>(s: &mut &'s str) -> ModalResult<&'s str> {"] # [doc = "   Caseless(\"hello\").parse_next(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"Hello, World!\"), Ok((\", World!\", \"Hello\")));"] # [doc = " assert_eq!(parser.parse_peek(\"hello, World!\"), Ok((\", World!\", \"hello\")));"] # [doc = " assert_eq!(parser.parse_peek(\"HeLlo, World!\"), Ok((\", World!\", \"HeLlo\")));"] # [doc = " assert!(parser.parse_peek(\"Some\").is_err());"] # [doc = " assert!(parser.parse_peek(\"\").is_err());"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct Caseless < T > (pub T) ;
};
}
