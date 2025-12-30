// Generated macro for backtrack_err (function)
macro_rules! Depcrate_combinator_corebacktrack_err {
() => {
// Module: crate::combinator::core
// Provides: {"backtrack_err"}
// Dependencies: {}
# [doc = " Transforms an [`ErrMode::Cut`][crate::error::ErrMode::Cut] (unrecoverable) to [`ErrMode::Backtrack`][crate::error::ErrMode::Backtrack] (recoverable)"] # [doc = ""] # [doc = " This attempts the parse, allowing other parsers to be tried on failure, like with"] # [doc = " [`winnow::combinator::alt`][crate::combinator::alt]."] pub fn backtrack_err < Input , Output , Error , ParseNext > (mut parser : ParseNext ,) -> impl Parser < Input , Output , Error > where Input : Stream , Error : ParserError < Input > + ModalError , ParseNext : Parser < Input , Output , Error > , { trace ("backtrack_err" , move | input : & mut Input | { parser . parse_next (input) . map_err (| e | e . backtrack ()) }) }
};
}
