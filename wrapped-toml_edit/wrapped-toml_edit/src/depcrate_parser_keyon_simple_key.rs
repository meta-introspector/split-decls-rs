// Generated macro for on_simple_key (function)
macro_rules! Depcrate_parser_keyon_simple_key {
() => {
// Module: crate::parser::key
// Provides: {"on_simple_key"}
// Dependencies: {}
# [doc = " ```bnf"] # [doc = " simple-key = quoted-key / unquoted-key"] # [doc = " quoted-key = basic-string / literal-string"] # [doc = " ```"] pub (crate) fn on_simple_key (event : & toml_parser :: parser :: Event , source : toml_parser :: Source < '_ > , errors : & mut dyn ErrorSink ,) -> (RawString , String) { # [cfg (feature = "debug")] let _scope = TraceScope :: new ("key::on_simple_key") ; let raw = source . get (event) . unwrap () ; let mut key = std :: borrow :: Cow :: Borrowed ("") ; raw . decode_key (& mut key , errors) ; let span = event . span () ; let raw = RawString :: with_span (span . start () .. span . end ()) ; let key = String :: from (key) ; (raw , key) }
};
}
