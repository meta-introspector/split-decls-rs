macro_rules! deps {
    () => {
        DeString!();
        State!();
        Error!();
        TraceScope!();
    };
}

macro_rules! on_key {
    () => {
        deps!();
        # [doc = " ```bnf"] # [doc = " key = simple-key / dotted-key"] # [doc = " dotted-key = simple-key 1*( dot-sep simple-key )"] # [doc = " ```"] pub (crate) fn on_key < 'i > (key_event : & toml_parser :: parser :: Event , input : & mut Input < '_ > , source : toml_parser :: Source < 'i > , errors : & mut dyn ErrorSink ,) -> (Vec < Spanned < DeString < 'i > > > , Option < Spanned < DeString < 'i > > >) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("key::on_key") ; let mut result_path = Vec :: new () ; let mut result_key = None ; let mut state = State :: new (key_event) ; if more_key (input) { while let Some (event) = input . next_token () { match event . kind () { EventKind :: StdTableOpen | EventKind :: ArrayTableOpen | EventKind :: InlineTableOpen | EventKind :: InlineTableClose | EventKind :: ArrayOpen | EventKind :: ArrayClose | EventKind :: Scalar | EventKind :: ValueSep | EventKind :: Comment | EventKind :: Newline | EventKind :: KeyValSep | EventKind :: StdTableClose | EventKind :: ArrayTableClose | EventKind :: Error => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; continue ; } EventKind :: SimpleKey => { state . current_key = Some (* event) ; if ! more_key (input) { break ; } } EventKind :: Whitespace => { state . whitespace (event) ; } EventKind :: KeySep => { state . close_key (& mut result_path , & mut result_key , source , errors) ; } } } } state . close_key (& mut result_path , & mut result_key , source , errors) ; # [cfg (not (feature = "unbounded"))] if super :: LIMIT <= result_path . len () as u32 { errors . report_error (ParseError :: new ("recursion limit")) ; return (Vec :: new () , None) ; } (result_path , result_key) }
    };
}

on_key!()