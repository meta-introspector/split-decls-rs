macro_rules! deps {
    () => {
        State!();
        RawString!();
        Entry!();
        Value!();
        InlineTable!();
        TraceScope!();
        Item!();
        Decor!();
        Key!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl State { fn whitespace (& mut self , event : & toml_parser :: parser :: Event) { let decor = if self . is_prefix () { self . current_prefix . get_or_insert (event . span ()) } else { self . current_suffix . get_or_insert (event . span ()) } ; * decor = decor . append (event . span ()) ; } fn is_prefix (& self) -> bool { if self . seen_keyval_sep { self . current_value . is_none () } else { self . current_key . is_none () } } fn capture_key (& mut self , event : & toml_parser :: parser :: Event , path : Vec < Key > , key : Option < Key > ,) { self . current_prefix . get_or_insert_with (| | event . span () . before ()) ; if let Some (key) = key { self . current_key = Some ((path , key)) ; } } fn finish_key (& mut self , event : & toml_parser :: parser :: Event) { self . seen_keyval_sep = true ; if let Some (last_key) = self . current_key . as_mut () . map (| (_ , k) | k) { let prefix = self . current_prefix . take () . expect ("setting a key should set a prefix") ; let suffix = self . current_suffix . take () . unwrap_or_else (| | event . span () . before ()) ; let prefix = RawString :: with_span (prefix . start () .. prefix . end ()) ; let suffix = RawString :: with_span (suffix . start () .. suffix . end ()) ; let leaf_decor = Decor :: new (prefix , suffix) ; * last_key . leaf_decor_mut () = leaf_decor ; } } fn capture_value (& mut self , event : & toml_parser :: parser :: Event , value : Value) { self . current_prefix . get_or_insert_with (| | event . span () . before ()) ; self . current_value = Some (value) ; } fn finish_value (& mut self , event : & toml_parser :: parser :: Event , result : & mut InlineTable , errors : & mut dyn ErrorSink ,) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::finish_value") ; self . seen_keyval_sep = false ; if let (Some ((path , key)) , Some (mut value)) = (self . current_key . take () , self . current_value . take ()) { let prefix = self . current_prefix . take () . expect ("setting a value should set a prefix") ; let suffix = self . current_suffix . take () . unwrap_or_else (| | event . span () . before ()) ; let Some (table) = descend_path (result , & path , true , errors) else { return ; } ; let decor = value . decor_mut () ; decor . set_prefix (RawString :: with_span (prefix . start () .. prefix . end ())) ; decor . set_suffix (RawString :: with_span (suffix . start () .. suffix . end ())) ; let mixed_table_types = table . is_dotted () == path . is_empty () ; if mixed_table_types { let key_span = get_key_span (& key) . unwrap_or_else (| | event . span ()) ; errors . report_error (ParseError :: new ("duplicate key") . with_unexpected (key_span)) ; } else { let key_span = get_key_span (& key) . unwrap_or_else (| | event . span ()) ; match table . items . entry (key) { Entry :: Vacant (o) => { o . insert (Item :: Value (value)) ; } Entry :: Occupied (o) => { let old_span = get_key_span (o . key ()) . unwrap_or_else (| | event . span ()) ; errors . report_error (ParseError :: new ("duplicate key") . with_unexpected (key_span) . with_context (old_span) ,) ; } } } } } fn close (& mut self , open_event : & toml_parser :: parser :: Event , close_event : & toml_parser :: parser :: Event , result : & mut InlineTable ,) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::close") ; let span = open_event . span () . append (close_event . span ()) ; let preamble = self . current_prefix . take () . map (| prefix | RawString :: with_span (prefix . start () .. prefix . end ())) ; result . span = Some (span . start () .. span . end ()) ; if let Some (preamble) = preamble { result . set_preamble (preamble) ; } } }
    };
}

impl_182!();