// Generated macro for eof (function)
macro_rules! Depcrate_combinator_coreeof {
() => {
// Module: crate::combinator::core
// Provides: {"eof"}
// Dependencies: {}
# [doc = " Match the end of the [`Stream`]"] # [doc = ""] # [doc = " Otherwise, it will error."] # [doc = ""] # [doc = " # Effective Signature"] # [doc = ""] # [doc = " Assuming you are parsing a `&str` [Stream]:"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;;"] # [doc = " pub fn eof<'i>(input: &mut &'i str) -> ModalResult<&'i str>"] # [doc = " # {"] # [doc = " #     winnow::combinator::eof.parse_next(input)"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use std::str;"] # [doc = " # use winnow::combinator::eof;"] # [doc = " # use winnow::prelude::*;"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<&'i str> {"] # [doc = "     eof.parse_next(input)"] # [doc = " }"] # [doc = " assert!(parser.parse_peek(\"abc\").is_err());"] # [doc = " assert_eq!(parser.parse_peek(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] # [doc (alias = "end")] # [doc (alias = "eoi")] pub fn eof < Input , Error > (input : & mut Input) -> Result < < Input as Stream > :: Slice , Error > where Input : Stream , Error : ParserError < Input > , { trace ("eof" , move | input : & mut Input | { if input . eof_offset () == 0 { Ok (input . next_slice (0)) } else { Err (ParserError :: from_input (input)) } }) . parse_next (input) }
};
}
