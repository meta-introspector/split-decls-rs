macro_rules! deps {
    () => {
        State!();
        Value!();
        Error!();
        TraceScope!();
        InlineTable!();
        Table!();
    };
}

macro_rules! on_inline_table {
    () => {
        deps!();
        # [doc = " ```bnf"] # [doc = " ;; Inline Table"] # [doc = ""] # [doc = " inline-table = inline-table-open inline-table-keyvals inline-table-close"] # [doc = " ```"] pub (crate) fn on_inline_table (open_event : & toml_parser :: parser :: Event , input : & mut Input < '_ > , source : toml_parser :: Source < '_ > , errors : & mut dyn ErrorSink ,) -> Value { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::on_inline_table") ; let mut result = InlineTable :: new () ; let mut state = State :: default () ; while let Some (event) = input . next_token () { match event . kind () { EventKind :: StdTableOpen | EventKind :: ArrayTableOpen | EventKind :: StdTableClose | EventKind :: ArrayClose | EventKind :: ArrayTableClose | EventKind :: KeySep => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; break ; } EventKind :: Error => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; continue ; } EventKind :: SimpleKey => { let (path , key) = on_key (event , input , source , errors) ; state . capture_key (event , path , key) ; } EventKind :: KeyValSep => { state . finish_key (event) ; } EventKind :: InlineTableOpen => { let value = on_inline_table (event , input , source , errors) ; state . capture_value (event , value) ; } EventKind :: ArrayOpen => { let value = on_array (event , input , source , errors) ; state . capture_value (event , value) ; } EventKind :: Scalar => { let value = on_scalar (event , source , errors) ; state . capture_value (event , value) ; } EventKind :: ValueSep => { state . finish_value (event , & mut result , errors) ; } EventKind :: Whitespace | EventKind :: Comment | EventKind :: Newline => { state . whitespace (event) ; } EventKind :: InlineTableClose => { state . finish_value (event , & mut result , errors) ; state . close (open_event , event , & mut result) ; break ; } } } Value :: InlineTable (result) }
    };
}

on_inline_table!();