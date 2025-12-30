// Generated macro for rest (function)
macro_rules! Depcrate_tokenrest {
() => {
// Module: crate::token
// Provides: {"rest"}
// Dependencies: {}
# [doc = " Return the remaining input."] # [doc = ""] # [doc = " # Effective Signature"] # [doc = ""] # [doc = " Assuming you are parsing a `&str` [Stream]:"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;;"] # [doc = " pub fn rest<'i>(input: &mut &'i str) -> ModalResult<&'i str>"] # [doc = " # {"] # [doc = " #     winnow::token::rest.parse_next(input)"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::error::ContextError;"] # [doc = " use winnow::token::rest;"] # [doc = " assert_eq!(rest::<_,ContextError>.parse_peek(\"abc\"), Ok((\"\", \"abc\")));"] # [doc = " assert_eq!(rest::<_,ContextError>.parse_peek(\"\"), Ok((\"\", \"\")));"] # [doc = " ```"] # [inline] pub fn rest < Input , Error > (input : & mut Input) -> Result < < Input as Stream > :: Slice , Error > where Input : Stream , Error : ParserError < Input > , { trace ("rest" , move | input : & mut Input | Ok (input . finish ())) . parse_next (input) }
};
}
