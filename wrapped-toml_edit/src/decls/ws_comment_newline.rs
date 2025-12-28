macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ws_comment_newline {
    () => {
        deps!();
        fn ws_comment_newline (input : & mut Input < '_ >) -> Option < toml_parser :: Span > { let mut current_span = None ; while let Some (event) = input . next_token () { match event . kind () { EventKind :: InlineTableOpen | EventKind :: InlineTableClose | EventKind :: ArrayOpen | EventKind :: ArrayClose | EventKind :: Scalar | EventKind :: ValueSep | EventKind :: Error | EventKind :: SimpleKey | EventKind :: KeySep | EventKind :: KeyValSep | EventKind :: StdTableOpen | EventKind :: ArrayTableOpen | EventKind :: StdTableClose | EventKind :: ArrayTableClose => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; } EventKind :: Whitespace | EventKind :: Comment => { let span = current_span . get_or_insert_with (| | event . span ()) ; * span = span . append (event . span ()) ; } EventKind :: Newline => { break ; } } } current_span }
    };
}

ws_comment_newline!();