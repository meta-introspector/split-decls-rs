// Generated macro for not (function)
macro_rules! Depcrate_combinator_corenot {
() => {
// Module: crate::combinator::core
// Provides: {"not"}
// Dependencies: {}
# [doc = " Succeeds if the child parser returns an error."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Note:** This does not advance the [`Stream`]"] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use winnow::prelude::*;"] # [doc = " use winnow::combinator::not;"] # [doc = " use winnow::ascii::alpha1;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " fn parser<'i>(input: &mut &'i str) -> ModalResult<()> {"] # [doc = "     not(alpha1).parse_next(input)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(parser.parse_peek(\"123\"), Ok((\"123\", ())));"] # [doc = " assert!(parser.parse_peek(\"abcd\").is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn not < Input , Output , Error , ParseNext > (mut parser : ParseNext) -> impl Parser < Input , () , Error > where Input : Stream , Error : ParserError < Input > , ParseNext : Parser < Input , Output , Error > , { trace ("not" , move | input : & mut Input | { let start = input . checkpoint () ; let res = parser . parse_next (input) ; input . reset (& start) ; match res { Ok (_) => Err (ParserError :: from_input (input)) , Err (e) if e . is_backtrack () => Ok (()) , Err (e) => Err (e) , } }) }
};
}
