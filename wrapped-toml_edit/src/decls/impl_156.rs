macro_rules! deps {
    () => {
        Array!();
        RawString!();
        State!();
        Value!();
        TraceScope!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl State { fn open (& mut self , open_event : & toml_parser :: parser :: Event) { self . trailing_start = Some (open_event . span () . end ()) ; } fn whitespace (& mut self , event : & toml_parser :: parser :: Event) { let decor = if self . is_prefix () { self . current_prefix . get_or_insert (event . span ()) } else { self . current_suffix . get_or_insert (event . span ()) } ; * decor = decor . append (event . span ()) ; } fn is_prefix (& self) -> bool { self . current_value . is_none () } fn capture_value (& mut self , event : & toml_parser :: parser :: Event , value : Value) { self . trailing_start = None ; self . current_prefix . get_or_insert_with (| | event . span () . before ()) ; self . current_value = Some (value) ; } fn finish_value (& mut self , event : & toml_parser :: parser :: Event , result : & mut Array) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("array::finish_value") ; if let Some (mut value) = self . current_value . take () { let prefix = self . current_prefix . take () . expect ("setting a value should set a prefix") ; let suffix = self . current_suffix . take () . unwrap_or_else (| | event . span () . before ()) ; let decor = value . decor_mut () ; decor . set_prefix (RawString :: with_span (prefix . start () .. prefix . end ())) ; decor . set_suffix (RawString :: with_span (suffix . start () .. suffix . end ())) ; result . push_formatted (value) ; } } fn sep_value (& mut self , event : & toml_parser :: parser :: Event) { self . trailing_start = Some (event . span () . end ()) ; } fn close (& mut self , open_event : & toml_parser :: parser :: Event , close_event : & toml_parser :: parser :: Event , result : & mut Array ,) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("array::close") ; let trailing_comma = self . trailing_start . is_some () && ! result . is_empty () ; let span = open_event . span () . append (close_event . span ()) ; let trailing_start = self . trailing_start . unwrap_or_else (| | close_event . span () . start ()) ; let trailing_end = close_event . span () . start () ; result . set_trailing_comma (trailing_comma) ; result . set_trailing (RawString :: with_span (trailing_start .. trailing_end)) ; result . span = Some (span . start () .. span . end ()) ; } }
    };
}

impl_156!();