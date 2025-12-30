// Generated macro for path_str (function)
macro_rules! Depcrate_attrpath_str {
() => {
// Module: crate::attr
// Provides: {"path_str"}
// Dependencies: {}
fn path_str (path : & Path) -> String { path . get_ident () . map_or_else (| | path . into_token_stream () . to_string () . replace (' ' , "") , Ident :: to_string ,) }
};
}
