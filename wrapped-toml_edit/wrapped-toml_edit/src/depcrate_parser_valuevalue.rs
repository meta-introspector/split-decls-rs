// Generated macro for value (function)
macro_rules! Depcrate_parser_valuevalue {
() => {
// Module: crate::parser::value
// Provides: {"value"}
// Dependencies: {}
# [doc = " ```bnf"] # [doc = " val = string / boolean / array / inline-table / date-time / float / integer"] # [doc = " ```"] pub (crate) fn value (input : & mut Input < '_ > , source : toml_parser :: Source < '_ > , errors : & mut dyn ErrorSink ,) -> Value { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("value") ; if let Some (event) = input . next_token () { match event . kind () { EventKind :: StdTableOpen | EventKind :: ArrayTableOpen | EventKind :: InlineTableClose | EventKind :: ArrayClose | EventKind :: ValueSep | EventKind :: Comment | EventKind :: Newline | EventKind :: Error | EventKind :: SimpleKey | EventKind :: KeySep | EventKind :: KeyValSep | EventKind :: StdTableClose | EventKind :: ArrayTableClose => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; } EventKind :: Whitespace => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; } EventKind :: InlineTableOpen => { return on_inline_table (event , input , source , errors) ; } EventKind :: ArrayOpen => { return on_array (event , input , source , errors) ; } EventKind :: Scalar => { return on_scalar (event , source , errors) ; } } } Value :: from (0) }
};
}
