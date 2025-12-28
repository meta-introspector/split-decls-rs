macro_rules! deps {
    () => {
        DeString!();
        DeValue!();
        TraceScope!();
        DeTable!();
        Entry!();
        State!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < 'i > State < 'i > { fn whitespace (& mut self , _event : & toml_parser :: parser :: Event) { } fn capture_key (& mut self , _event : & toml_parser :: parser :: Event , path : Vec < Spanned < DeString < 'i > > > , key : Option < Spanned < DeString < 'i > > > ,) { if let Some (key) = key { self . current_key = Some ((path , key)) ; } } fn finish_key (& mut self , _event : & toml_parser :: parser :: Event) { self . seen_keyval_sep = true ; } fn capture_value (& mut self , _event : & toml_parser :: parser :: Event , value : Spanned < DeValue < 'i > >) { self . current_value = Some (value) ; } fn finish_value (& mut self , _event : & toml_parser :: parser :: Event , result : & mut DeTable < 'i > , errors : & mut dyn ErrorSink ,) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::finish_value") ; self . seen_keyval_sep = false ; if let (Some ((path , key)) , Some (value)) = (self . current_key . take () , self . current_value . take ()) { let Some (table) = descend_path (result , & path , true , errors) else { return ; } ; let mixed_table_types = table . is_dotted () == path . is_empty () ; if mixed_table_types { let key_span = get_key_span (& key) ; errors . report_error (ParseError :: new ("duplicate key") . with_unexpected (key_span)) ; } else { let key_span = get_key_span (& key) ; match table . entry (key) { Entry :: Vacant (o) => { o . insert (value) ; } Entry :: Occupied (o) => { let old_span = get_key_span (o . key ()) ; errors . report_error (ParseError :: new ("duplicate key") . with_unexpected (key_span) . with_context (old_span) ,) ; } } } } } fn close (& mut self , _open_event : & toml_parser :: parser :: Event , _close_event : & toml_parser :: parser :: Event , _result : & mut DeTable < 'i > ,) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("inline_table::close") ; } }
    };
}

impl_245!();