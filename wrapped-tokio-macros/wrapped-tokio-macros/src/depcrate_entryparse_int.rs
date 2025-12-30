// Generated macro for parse_int (function)
macro_rules! Depcrate_entryparse_int {
() => {
// Module: crate::entry
// Provides: {"parse_int"}
// Dependencies: {}
fn parse_int (int : syn :: Lit , span : Span , field : & str) -> Result < usize , syn :: Error > { match int { syn :: Lit :: Int (lit) => match lit . base10_parse :: < usize > () { Ok (value) => Ok (value) , Err (e) => Err (syn :: Error :: new (span , format ! ("Failed to parse value of `{field}` as integer: {e}") ,)) , } , _ => Err (syn :: Error :: new (span , format ! ("Failed to parse value of `{field}` as integer.") ,)) , } }
};
}
