// Generated macro for empty (function)
macro_rules! Depcrate_combinator_coreempty {
() => {
// Module: crate::combinator::core
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Succeed, consuming no input"] # [doc = ""] # [doc = " For example, it can be used as the last alternative in `alt` to"] # [doc = " specify the default case."] # [doc = ""] # [doc = " Useful with:"] # [doc = " - [`Parser::value`]"] # [doc = " - [`Parser::default_value`]"] # [doc = " - [`Parser::map`]"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Note:** This never advances the [`Stream`]"] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::alt;"] # [doc = " use winnow::combinator::empty;"] # [doc = ""] # [doc = " fn sign(input: &mut &str) -> ModalResult<isize> {"] # [doc = "     alt(("] # [doc = "         '-'.value(-1),"] # [doc = "         '+'.value(1),"] # [doc = "         empty.value(1)"] # [doc = "     )).parse_next(input)"] # [doc = " }"] # [doc = " assert_eq!(sign.parse_peek(\"+10\"), Ok((\"10\", 1)));"] # [doc = " assert_eq!(sign.parse_peek(\"-10\"), Ok((\"10\", -1)));"] # [doc = " assert_eq!(sign.parse_peek(\"10\"), Ok((\"10\", 1)));"] # [doc = " ```"] # [doc (alias = "value")] # [doc (alias = "success")] # [inline] pub fn empty < Input , Error > (_input : & mut Input) -> Result < () , Error > where Input : Stream , Error : ParserError < Input > , { Ok (()) }
};
}
