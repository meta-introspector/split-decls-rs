// Generated macro for parse_string (function)
macro_rules! Depcrate_entryparse_string {
() => {
// Module: crate::entry
// Provides: {"parse_string"}
// Dependencies: {}
fn parse_string (int : syn :: Lit , span : Span , field : & str) -> Result < String , syn :: Error > { match int { syn :: Lit :: Str (s) => Ok (s . value ()) , syn :: Lit :: Verbatim (s) => Ok (s . to_string ()) , _ => Err (syn :: Error :: new (span , format ! ("Failed to parse value of `{field}` as string.") ,)) , } }
};
}
