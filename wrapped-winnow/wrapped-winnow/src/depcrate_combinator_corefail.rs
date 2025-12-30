// Generated macro for fail (function)
macro_rules! Depcrate_combinator_corefail {
() => {
// Module: crate::combinator::core
// Provides: {"fail"}
// Dependencies: {}
# [doc = " A parser which always fails."] # [doc = ""] # [doc = " For example, it can be used as the last alternative in `alt` to"] # [doc = " control the error message given."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::{error::ErrMode, error::InputError};"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::fail;"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<(), InputError<&'i str>> {"] # [doc = "     fail.parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"string\"), Err(ErrMode::Backtrack(InputError::at(\"string\"))));"] # [doc = " ```"] # [doc (alias = "unexpected")] # [inline] pub fn fail < Input , Output , Error > (i : & mut Input) -> Result < Output , Error > where Input : Stream , Error : ParserError < Input > , { trace ("fail" , | i : & mut Input | Err (ParserError :: from_input (i))) . parse_next (i) }
};
}
