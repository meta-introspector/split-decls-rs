macro_rules! deps {
    () => {
        InlineTable!();
        TraceScope!();
        Key!();
        InlineEntry!();
        Value!();
    };
}

macro_rules! descend_path {
    () => {
        deps!();
        fn descend_path < 'a > (mut table : & 'a mut InlineTable , path : & 'a [Key] , dotted : bool , errors : & mut dyn ErrorSink ,) -> Option < & 'a mut InlineTable > { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::descend_path") ; # [cfg (feature = "debug")] trace (& format ! ("key={:?}" , path . iter () . map (| k | k . get ()) . collect ::< Vec < _ >> ()) , anstyle :: AnsiColor :: Blue . on_default () ,) ; for key in path . iter () { table = match table . entry_format (key) { crate :: InlineEntry :: Vacant (entry) => { let mut new_table = InlineTable :: new () ; new_table . span = key . span () ; new_table . set_implicit (true) ; new_table . set_dotted (dotted) ; entry . insert (Value :: InlineTable (new_table)) . as_inline_table_mut () . unwrap () } crate :: InlineEntry :: Occupied (entry) => { match entry . into_mut () { Value :: InlineTable (sweet_child_of_mine) => { if dotted && ! sweet_child_of_mine . is_implicit () { let key_span = get_key_span (key) . expect ("all keys have spans") ; errors . report_error (ParseError :: new ("duplicate key") . with_unexpected (key_span) ,) ; return None ; } sweet_child_of_mine } item => { let key_span = get_key_span (key) . expect ("all keys have spans") ; errors . report_error (ParseError :: new (format ! ("cannot extend value of type {} with a dotted key" , item . type_name ())) . with_unexpected (key_span) ,) ; return None ; } } } } ; } Some (table) }
    };
}

descend_path!();