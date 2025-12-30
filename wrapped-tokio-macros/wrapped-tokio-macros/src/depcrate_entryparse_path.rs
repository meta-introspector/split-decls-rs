// Generated macro for parse_path (function)
macro_rules! Depcrate_entryparse_path {
() => {
// Module: crate::entry
// Provides: {"parse_path"}
// Dependencies: {}
fn parse_path (lit : syn :: Lit , span : Span , field : & str) -> Result < Path , syn :: Error > { match lit { syn :: Lit :: Str (s) => { let err = syn :: Error :: new (span , format ! ("Failed to parse value of `{}` as path: \"{}\"" , field , s . value ()) ,) ; s . parse :: < syn :: Path > () . map_err (| _ | err . clone ()) } _ => Err (syn :: Error :: new (span , format ! ("Failed to parse value of `{field}` as path.") ,)) , } }
};
}
