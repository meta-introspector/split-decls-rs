// Generated macro for parsing (module)
macro_rules! Depcrate_litparsing {
() => {
// Module: crate::lit
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub mod parsing { use super :: * ; use synom :: { Synom , PResult , Cursor , parse_error } ; impl Synom for Lit { fn parse (input : Cursor) -> PResult < Self > { match input . literal () { Some ((rest , span , lit)) => { Ok ((rest , Lit { span : Span (span) , value : LitKind :: Other (lit) })) } _ => match input . word () { Some ((rest , span , sym)) => { let kind = if sym . as_str () == "true" { LitKind :: Bool (true) } else if sym . as_str () == "false" { LitKind :: Bool (false) } else { return parse_error () ; } ; Ok ((rest , Lit { span : Span (span) , value : kind })) } _ => parse_error () , } } } } }
};
}
