// Generated macro for parse_value (function)
macro_rules! Depcrate_parserparse_value {
() => {
// Module: crate::parser
// Provides: {"parse_value"}
// Dependencies: {}
pub (crate) fn parse_value (source : toml_parser :: Source < '_ > , errors : & mut dyn prelude :: ErrorSink ,) -> crate :: Value { let tokens = source . lex () . into_vec () ; let mut events = Vec :: with_capacity (tokens . len ()) ; let mut receiver = ValidateWhitespace :: new (& mut events , source) ; # [cfg (not (feature = "unbounded"))] let mut receiver = RecursionGuard :: new (& mut receiver , LIMIT) ; # [cfg (not (feature = "unbounded"))] let receiver = & mut receiver ; # [cfg (feature = "unbounded")] let receiver = & mut receiver ; toml_parser :: parser :: parse_value (& tokens , receiver , errors) ; let mut input = prelude :: Input :: new (& events) ; let value = value :: value (& mut input , source , errors) ; value }
};
}
