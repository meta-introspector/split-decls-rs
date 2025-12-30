// Generated macro for value (function)
macro_rules! Depcrate_de_parser_valuevalue {
() => {
// Module: crate::de::parser::value
// Provides: {"value"}
// Dependencies: {}
# [doc = " ```bnf"] # [doc = " val = string / boolean / array / inline-table / date-time / float / integer"] # [doc = " ```"] pub (crate) fn value < 'i > (input : & mut Input < '_ > , source : toml_parser :: Source < 'i > , errors : & mut dyn ErrorSink ,) -> Spanned < DeValue < 'i > > { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("value") ; if let Some (event) = input . next_token () { match event . kind () { EventKind :: StdTableOpen | EventKind :: ArrayTableOpen | EventKind :: InlineTableClose | EventKind :: ArrayClose | EventKind :: ValueSep | EventKind :: Comment | EventKind :: Newline | EventKind :: Error | EventKind :: SimpleKey | EventKind :: KeySep | EventKind :: KeyValSep | EventKind :: StdTableClose | EventKind :: ArrayTableClose => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; } EventKind :: Whitespace => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; } EventKind :: InlineTableOpen => { return on_inline_table (event , input , source , errors) ; } EventKind :: ArrayOpen => { return on_array (event , input , source , errors) ; } EventKind :: Scalar => { return on_scalar (event , source , errors) ; } } } Spanned :: new (0 .. 0 , DeValue :: Integer (Default :: default ())) }
};
}
