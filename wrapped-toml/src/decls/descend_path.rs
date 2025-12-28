macro_rules! deps {
    () => {
        DeValue!();
        DeString!();
        Table!();
        DeTable!();
        TraceScope!();
        Entry!();
    };
}

macro_rules! descend_path {
    () => {
        deps!();
        fn descend_path < 'a , 'i > (mut table : & 'a mut DeTable < 'i > , path : & 'a [Spanned < DeString < 'i > >] , dotted : bool , errors : & mut dyn ErrorSink ,) -> Option < & 'a mut DeTable < 'i > > { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::descend_path") ; # [cfg (feature = "debug")] trace (& format ! ("key={:?}" , path . iter () . map (| k | k . get_ref ()) . collect ::< Vec < _ >> ()) , anstyle :: AnsiColor :: Blue . on_default () ,) ; for key in path . iter () { table = match table . entry (key . clone ()) { Entry :: Vacant (entry) => { let mut new_table = DeTable :: new () ; new_table . set_implicit (true) ; new_table . set_dotted (dotted) ; new_table . set_inline (true) ; let value = DeValue :: Table (new_table) ; let value = Spanned :: new (key . span () , value) ; let value = entry . insert (value) ; value . as_mut () . as_table_mut () . unwrap () } Entry :: Occupied (entry) => { let spanned = entry . into_mut () ; match spanned . as_mut () { DeValue :: Table (sweet_child_of_mine) => { if dotted && ! sweet_child_of_mine . is_implicit () { let key_span = get_key_span (key) ; errors . report_error (ParseError :: new ("duplicate key") . with_unexpected (key_span) ,) ; return None ; } sweet_child_of_mine } item => { let key_span = get_key_span (key) ; errors . report_error (ParseError :: new (format ! ("cannot extend value of type {} with a dotted key" , item . type_str ())) . with_unexpected (key_span) ,) ; return None ; } } } } ; } Some (table) }
    };
}

descend_path!()