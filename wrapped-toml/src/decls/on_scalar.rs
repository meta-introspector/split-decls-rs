macro_rules! deps {
    () => {
        DeInteger!();
        DeValue!();
        TraceScope!();
        DeFloat!();
    };
}

macro_rules! on_scalar {
    () => {
        deps!();
        pub (crate) fn on_scalar < 'i > (event : & toml_parser :: parser :: Event , source : toml_parser :: Source < 'i > , errors : & mut dyn ErrorSink ,) -> Spanned < DeValue < 'i > > { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("on_scalar") ; let value_span = event . span () ; let value_span = value_span . start () .. value_span . end () ; let raw = source . get (event) . unwrap () ; let mut decoded = alloc :: borrow :: Cow :: Borrowed ("") ; let kind = raw . decode_scalar (& mut decoded , errors) ; match kind { toml_parser :: decoder :: ScalarKind :: String => { Spanned :: new (value_span , DeValue :: String (decoded)) } toml_parser :: decoder :: ScalarKind :: Boolean (value) => { Spanned :: new (value_span , DeValue :: Boolean (value)) } toml_parser :: decoder :: ScalarKind :: DateTime => { let value = match decoded . parse :: < toml_datetime :: Datetime > () { Ok (value) => value , Err (err) => { errors . report_error (ParseError :: new (err . to_string ()) . with_unexpected (event . span ()) ,) ; toml_datetime :: Datetime { date : None , time : None , offset : None , } } } ; Spanned :: new (value_span , DeValue :: Datetime (value)) } toml_parser :: decoder :: ScalarKind :: Float => { Spanned :: new (value_span , DeValue :: Float (DeFloat { inner : decoded })) } toml_parser :: decoder :: ScalarKind :: Integer (radix) => Spanned :: new (value_span , DeValue :: Integer (DeInteger { inner : decoded , radix : radix . value () , }) ,) , } }
    };
}

on_scalar!();