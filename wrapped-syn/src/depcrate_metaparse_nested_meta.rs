// Generated macro for parse_nested_meta (function)
macro_rules! Depcrate_metaparse_nested_meta {
() => {
// Module: crate::meta
// Provides: {"parse_nested_meta"}
// Dependencies: {}
pub (crate) fn parse_nested_meta (input : ParseStream , mut logic : impl FnMut (ParseNestedMeta) -> Result < () > ,) -> Result < () > { loop { let path = input . call (parse_meta_path) ? ; logic (ParseNestedMeta { path , input }) ? ; if input . is_empty () { return Ok (()) ; } input . parse :: < Token ! [,] > () ? ; if input . is_empty () { return Ok (()) ; } } }
};
}
