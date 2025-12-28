macro_rules! deps {
    () => {
        Key!();
        RawString!();
        Repr!();
        State!();
        Decor!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl State { fn new (key_event : & toml_parser :: parser :: Event) -> Self { Self { current_prefix : None , current_key : Some (* key_event) , current_suffix : None , } } fn whitespace (& mut self , event : & toml_parser :: parser :: Event) { if self . current_key . is_some () { self . current_suffix = Some (event . span ()) ; } else { self . current_prefix = Some (event . span ()) ; } } fn close_key (& mut self , result_path : & mut Vec < Key > , result_key : & mut Option < Key > , source : toml_parser :: Source < '_ > , errors : & mut dyn ErrorSink ,) { let Some (key) = self . current_key . take () else { return ; } ; let prefix_span = self . current_prefix . take () . unwrap_or_else (| | key . span () . before ()) ; let prefix = RawString :: with_span (prefix_span . start () .. prefix_span . end ()) ; let suffix_span = self . current_suffix . take () . unwrap_or_else (| | key . span () . after ()) ; let suffix = RawString :: with_span (suffix_span . start () .. suffix_span . end ()) ; let key_span = key . span () ; let key_raw = RawString :: with_span (key_span . start () .. key_span . end ()) ; let raw = source . get (key) . unwrap () ; let mut decoded = std :: borrow :: Cow :: Borrowed ("") ; raw . decode_key (& mut decoded , errors) ; let key = Key :: new (decoded) . with_repr_unchecked (Repr :: new_unchecked (key_raw)) . with_dotted_decor (Decor :: new (prefix , suffix)) ; if let Some (last_key) = result_key . replace (key) { result_path . push (last_key) ; } } }
    };
}

impl_189!();