// Generated macro for parse_key (function)
macro_rules! Depcrate_parserparse_key {
() => {
// Module: crate::parser
// Provides: {"parse_key"}
// Dependencies: {}
pub (crate) fn parse_key (source : toml_parser :: Source < '_ > , errors : & mut dyn prelude :: ErrorSink ,) -> crate :: Key { let tokens = source . lex () . into_vec () ; let mut events = Vec :: with_capacity (tokens . len ()) ; let mut receiver = ValidateWhitespace :: new (& mut events , source) ; # [cfg (not (feature = "unbounded"))] let mut receiver = RecursionGuard :: new (& mut receiver , LIMIT) ; # [cfg (not (feature = "unbounded"))] let receiver = & mut receiver ; # [cfg (feature = "unbounded")] let receiver = & mut receiver ; toml_parser :: parser :: parse_simple_key (& tokens , receiver , errors) ; if let Some (event) = events . iter () . find (| e | e . kind () == toml_parser :: parser :: EventKind :: SimpleKey) { let (raw , key) = key :: on_simple_key (event , source , errors) ; crate :: Key :: new (key) . with_repr_unchecked (crate :: Repr :: new_unchecked (raw)) } else { let key = source . input () ; let raw = RawString :: with_span (0 .. source . input () . len ()) ; crate :: Key :: new (key) . with_repr_unchecked (crate :: Repr :: new_unchecked (raw)) } }
};
}
