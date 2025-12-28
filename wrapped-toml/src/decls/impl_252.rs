macro_rules! deps {
    () => {
        State!();
        DeString!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl State { fn new (key_event : & toml_parser :: parser :: Event) -> Self { Self { current_key : Some (* key_event) , } } fn whitespace (& mut self , _event : & toml_parser :: parser :: Event) { } fn close_key < 'i > (& mut self , result_path : & mut Vec < Spanned < DeString < 'i > > > , result_key : & mut Option < Spanned < DeString < 'i > > > , source : toml_parser :: Source < 'i > , errors : & mut dyn ErrorSink ,) { let Some (key) = self . current_key . take () else { return ; } ; let key_span = key . span () ; let key_span = key_span . start () .. key_span . end () ; let raw = source . get (key) . unwrap () ; let mut decoded = alloc :: borrow :: Cow :: Borrowed ("") ; raw . decode_key (& mut decoded , errors) ; let key = Spanned :: new (key_span , decoded) ; if let Some (last_key) = result_key . replace (key) { result_path . push (last_key) ; } } }
    };
}

impl_252!()