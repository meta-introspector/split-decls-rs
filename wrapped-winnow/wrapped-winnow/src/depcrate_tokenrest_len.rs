// Generated macro for rest_len (function)
macro_rules! Depcrate_tokenrest_len {
() => {
// Module: crate::token
// Provides: {"rest_len"}
// Dependencies: {}
# [doc = " Return the length of the remaining input."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Note: this does not advance the [`Stream`]"] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Effective Signature"] # [doc = ""] # [doc = " Assuming you are parsing a `&str` [Stream]:"] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;;"] # [doc = " pub fn rest_len(input: &mut &str) -> ModalResult<usize>"] # [doc = " # {"] # [doc = " #     winnow::token::rest_len.parse_next(input)"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " # use winnow::error::ContextError;"] # [doc = " use winnow::token::rest_len;"] # [doc = " assert_eq!(rest_len::<_,ContextError>.parse_peek(\"abc\"), Ok((\"abc\", 3)));"] # [doc = " assert_eq!(rest_len::<_,ContextError>.parse_peek(\"\"), Ok((\"\", 0)));"] # [doc = " ```"] # [inline] pub fn rest_len < Input , Error > (input : & mut Input) -> Result < usize , Error > where Input : Stream , Error : ParserError < Input > , { trace ("rest_len" , move | input : & mut Input | { let len = input . eof_offset () ; Ok (len) }) . parse_next (input) }
};
}
