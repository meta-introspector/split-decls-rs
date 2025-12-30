// Generated macro for parsing (module)
macro_rules! Depcrate_lifetimeparsing {
() => {
// Module: crate::lifetime
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub mod parsing { use super :: * ; use synom :: { Synom , PResult , Cursor , parse_error } ; impl Synom for Lifetime { fn parse (input : Cursor) -> PResult < Self > { let (rest , span , sym) = match input . word () { Some (word) => word , _ => return parse_error () , } ; if ! sym . as_str () . starts_with ('\'') { return parse_error () ; } Ok ((rest , Lifetime { sym : sym , span : Span (span) , })) } } }
};
}
