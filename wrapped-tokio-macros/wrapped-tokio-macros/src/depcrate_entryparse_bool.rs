// Generated macro for parse_bool (function)
macro_rules! Depcrate_entryparse_bool {
() => {
// Module: crate::entry
// Provides: {"parse_bool"}
// Dependencies: {}
fn parse_bool (bool : syn :: Lit , span : Span , field : & str) -> Result < bool , syn :: Error > { match bool { syn :: Lit :: Bool (b) => Ok (b . value) , _ => Err (syn :: Error :: new (span , format ! ("Failed to parse value of `{field}` as bool.") ,)) , } }
};
}
