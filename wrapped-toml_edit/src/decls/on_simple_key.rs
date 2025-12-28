macro_rules! deps {
    () => {
        RawString!();
        TraceScope!();
    };
}

macro_rules! on_simple_key {
    () => {
        deps!();
        # [doc = " ```bnf"] # [doc = " simple-key = quoted-key / unquoted-key"] # [doc = " quoted-key = basic-string / literal-string"] # [doc = " ```"] pub (crate) fn on_simple_key (event : & toml_parser :: parser :: Event , source : toml_parser :: Source < '_ > , errors : & mut dyn ErrorSink ,) -> (RawString , String) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("key::on_simple_key") ; let raw = source . get (event) . unwrap () ; let mut key = std :: borrow :: Cow :: Borrowed ("") ; raw . decode_key (& mut key , errors) ; let span = event . span () ; let raw = RawString :: with_span (span . start () .. span . end ()) ; let key = String :: from (key) ; (raw , key) }
    };
}

on_simple_key!()