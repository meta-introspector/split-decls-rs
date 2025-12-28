macro_rules! deps {
    () => {
        Array!();
        Error!();
        TraceScope!();
        Decor!();
        Table!();
        RawString!();
        TableHeader!();
    };
}

macro_rules! on_table {
    () => {
        deps!();
        # [doc = " ```bnf"] # [doc = " ;; Standard Table"] # [doc = ""] # [doc = " std-table = std-table-open key *( table-key-sep key) std-table-close"] # [doc = ""] # [doc = " ;; Array Table"] # [doc = ""] # [doc = " array-table = array-table-open key *( table-key-sep key) array-table-close"] # [doc = " ```"] fn on_table (open_event : & toml_parser :: parser :: Event , input : & mut Input < '_ > , source : toml_parser :: Source < '_ > , errors : & mut dyn ErrorSink ,) -> TableHeader { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("document::on_table") ; let is_array = open_event . kind () == EventKind :: ArrayTableOpen ; let mut current_path = None ; let mut current_key = None ; let mut current_span = open_event . span () ; let mut current_prefix = None ; let mut current_suffix = None ; while let Some (event) = input . next_token () { match event . kind () { EventKind :: InlineTableOpen | EventKind :: InlineTableClose | EventKind :: ArrayOpen | EventKind :: ArrayClose | EventKind :: Scalar | EventKind :: ValueSep | EventKind :: Error | EventKind :: KeySep | EventKind :: KeyValSep | EventKind :: StdTableOpen | EventKind :: ArrayTableOpen | EventKind :: Comment | EventKind :: Newline => { # [cfg (feature = "debug")] trace (& format ! ("unexpected {event:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; continue ; } EventKind :: ArrayTableClose | EventKind :: StdTableClose => { current_span = current_span . append (event . span ()) ; break ; } EventKind :: SimpleKey => { current_prefix . get_or_insert_with (| | event . span () . before ()) ; let (path , key) = on_key (event , input , source , errors) ; current_path = Some (path) ; current_key = key ; current_suffix . get_or_insert_with (| | event . span () . after ()) ; } EventKind :: Whitespace => { if current_key . is_some () { current_suffix = Some (event . span ()) ; } else { current_prefix = Some (event . span ()) ; } } } } let prefix = current_prefix . take () . expect ("setting a key should set a prefix") ; let suffix = current_suffix . take () . expect ("setting a key should set a suffix") ; if let Some (last_key) = current_key . as_mut () { let prefix = RawString :: with_span (prefix . start () .. prefix . end ()) ; let suffix = RawString :: with_span (suffix . start () .. suffix . end ()) ; let leaf_decor = Decor :: new (prefix , suffix) ; * last_key . leaf_decor_mut () = leaf_decor ; } TableHeader { path : current_path . unwrap_or_default () , key : current_key , span : current_span , is_array , } }
    };
}

on_table!();