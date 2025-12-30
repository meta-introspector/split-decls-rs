// Generated macro for helpers_parse_macro_delimiter (macro)
macro_rules! Depcrate_helpershelpers_parse_macro_delimiter {
() => {
// Module: crate::helpers
// Provides: {"helpers_parse_macro_delimiter"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! helpers_parse_macro_delimiter { ($ content : ident in $ input : ident) => { match $ crate :: helpers :: parse_macro_delimiter ($ input) { Ok ((token , content)) => { $ content = content ; token } Err (e) => return Err (e) , } } ; }
};
}
